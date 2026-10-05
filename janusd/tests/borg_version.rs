// SPDX-License-Identifier: GPL-3.0-or-later

//! Version pin checks for both Borg series (stubs, real processes).

mod borg_common;

use janusd::janus_borg::{BorgSeries, probe_version, series_from_version_string};

#[tokio::test]
async fn probe_accepts_v1_pin() {
	let dir = borg_common::temp_dir("v1");
	let bin = borg_common::stub_bor(&dir, "borg", "#!/bin/sh\necho 'borg 1.4.5'\n");
	let found = probe_version(bin.to_str().unwrap(), BorgSeries::V1_4).await.unwrap();
	assert_eq!(found, "borg 1.4.5");
}

#[tokio::test]
async fn probe_accepts_v2_pin() {
	let dir = borg_common::temp_dir("v2");
	let bin = borg_common::stub_bor(&dir, "borg", "#!/bin/sh\necho 'borg 2.0.0b13'\n");
	let found = probe_version(bin.to_str().unwrap(), BorgSeries::V2_0B).await.unwrap();
	assert_eq!(found, "borg 2.0.0b13");
}

#[tokio::test]
async fn probe_rejects_foreign_version() {
	let dir = borg_common::temp_dir("foreign");
	let bin = borg_common::stub_bor(&dir, "borg", "#!/bin/sh\necho 'borg 1.2.3'\n");
	let err = probe_version(bin.to_str().unwrap(), BorgSeries::V1_4)
		.await
		.unwrap_err();
	assert!(err.to_string().contains("1.2.3"), "err: {err}");
	assert!(err.to_string().contains("1.4."), "err: {err}");
}

#[test]
fn series_mapping_is_exclusive() {
	assert_eq!(series_from_version_string("borg 1.4.0"), Some(BorgSeries::V1_4));
	assert_eq!(series_from_version_string("borg 2.0.0b25"), Some(BorgSeries::V2_0B));
	assert_eq!(series_from_version_string("borg 3.0.0b1"), None);
	assert_eq!(series_from_version_string("müll"), None);
}
