// SPDX-License-Identifier: GPL-3.0-or-later

//! Cancel ladder against TERM-respecting and stubborn stub children.

mod borg_common;

use std::os::unix::process::ExitStatusExt;
use std::time::{Duration, Instant};

use janusd::janus_borg::{BorgChild, BorgCmd, BorgSeries};

#[tokio::test]
async fn cancel_graceful_terminates_before_grace() {
	let dir = borg_common::temp_dir("graceful");
	let bin = borg_common::stub_bor(&dir, "borg", borg_common::sleep_stub_body());
	let mut child = BorgChild::spawn(BorgCmd::new(bin.to_str().unwrap(), BorgSeries::V1_4).build()).unwrap();
	assert_eq!(child.next_stdout_line().await.unwrap().as_deref(), Some("ready"));
	child.set_grace(Duration::from_secs(2));
	let started = Instant::now();
	let status = child.cancel().await.unwrap();
	assert!(status.success());
	assert!(
		started.elapsed() < Duration::from_secs(2),
		"took {:?}",
		started.elapsed()
	);
}

#[tokio::test]
async fn cancel_stubborn_escalates_to_sigkill() {
	let dir = borg_common::temp_dir("stubborn");
	let body = "#!/bin/sh\necho ready\ntrap '' TERM\nwhile true; do sleep 0.05; done\n";
	let bin = borg_common::stub_bor(&dir, "borg", body);
	let mut child = BorgChild::spawn(BorgCmd::new(bin.to_str().unwrap(), BorgSeries::V2_0B).build()).unwrap();
	assert_eq!(child.next_stdout_line().await.unwrap().as_deref(), Some("ready"));
	child.set_grace(Duration::from_millis(300));
	let started = Instant::now();
	let status = child.cancel().await.unwrap();
	assert_eq!(status.signal(), Some(9));
	assert!(started.elapsed() >= Duration::from_millis(300));
}
