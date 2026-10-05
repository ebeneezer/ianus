// SPDX-License-Identifier: GPL-3.0-or-later

//! Cancel after a finished child must be deterministic (no stray signal).

mod borg_common;

use janusd::janus_borg::{BorgChild, BorgCmd, BorgSeries};

#[tokio::test]
async fn cancel_after_finish_is_deterministic() {
	let dir = borg_common::temp_dir("after");
	let bin = borg_common::stub_bor(&dir, "borg", "#!/bin/sh\nexit 7\n");
	let mut child = BorgChild::spawn(BorgCmd::new(bin.to_str().unwrap(), BorgSeries::V1_4).build()).unwrap();
	let err = child.finish().await.unwrap_err();
	assert!(err.to_string().contains('7'), "err: {err}");
	let status = child.cancel().await.unwrap();
	assert_eq!(status.code(), Some(7));
}
