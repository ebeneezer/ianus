// SPDX-License-Identifier: GPL-3.0-or-later

//! Bounded line streaming of JSONL output (stub, real process).

mod borg_common;

use janusd::janus_borg::{BorgChild, BorgCmd, BorgSeries};

#[tokio::test]
async fn streams_jsonl_lines_until_eof() {
	let dir = borg_common::temp_dir("stream");
	let mut body = String::from("#!/bin/sh\n");
	for i in 1..=5 {
		body.push_str(&format!("echo '{{\"n\": {i}}}';\n"));
	}
	let bin = borg_common::stub_bor(&dir, "borg", &body);
	let cmd = BorgCmd::new(bin.to_str().unwrap(), BorgSeries::V1_4).file_listing();
	let mut child = BorgChild::spawn(cmd.build()).unwrap();
	let mut got = Vec::new();
	while let Some(line) = child.next_stdout_line().await.unwrap() {
		got.push(line);
	}
	assert_eq!(got.len(), 5);
	assert_eq!(got[0], "{\"n\": 1}");
	assert_eq!(got[4], "{\"n\": 5}");
	assert!(child.next_stdout_line().await.unwrap().is_none());
	child.finish().await.unwrap();
}
