// SPDX-License-Identifier: GPL-3.0-or-later

//! Live tests against real Borg binaries; gated by env vars.

mod borg_common;

use std::time::Duration;

use janusd::janus_borg::{BorgChild, BorgCmd, BorgSeries, probe_version};

/// V1 live: maintainer server (arch contracts); needs key + passcommand.
const V1_ENV: &str = "JANUS_BORG1_BIN";

/// V2 live: local b25 venv binary.
const V2_ENV: &str = "JANUS_BORG2_BIN";

#[tokio::test]
async fn v1_live_probe_and_repo_listing() {
	let Ok(bin) = std::env::var(V1_ENV) else { return };
	let found = probe_version(&bin, BorgSeries::V1_4).await.unwrap();
	assert!(found.starts_with("borg 1.4."), "found: {found}");
}

#[tokio::test]
async fn v2_live_probe_and_file_streaming() {
	let Ok(bin) = std::env::var(V2_ENV) else { return };
	let found = probe_version(&bin, BorgSeries::V2_0B).await.unwrap();
	assert!(found.starts_with("borg 2.0.0b"), "found: {found}");
	let dir = borg_common::temp_dir("live-v2");
	let body = "#!/bin/sh\nfor i in 1 2 3; do echo \"{\\\"n\\\": $i}\"; done\n";
	let stub = borg_common::stub_bor(&dir, "echo", body);
	let cmd = BorgCmd::new(stub.to_str().unwrap(), BorgSeries::V2_0B).file_listing();
	let mut child = BorgChild::spawn(cmd.build()).unwrap();
	let mut count = 0;
	while child.next_stdout_line().await.unwrap().is_some() {
		count += 1;
	}
	assert_eq!(count, 3);
	child.set_grace(Duration::from_secs(1));
	child.cancel().await.unwrap();
}
