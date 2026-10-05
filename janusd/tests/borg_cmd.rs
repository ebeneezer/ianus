// SPDX-License-Identifier: GPL-3.0-or-later

//! Argument construction per series (stub echoes its argv, real process).

mod borg_common;

use janusd::janus_borg::{BorgChild, BorgCmd, BorgSeries};

/// Spawns a command whose stdout echoes all argv lines; returns them.
async fn echo_argv(cmd: BorgCmd) -> Vec<String> {
	let mut child = BorgChild::spawn(cmd.build()).unwrap();
	let mut got = Vec::new();
	while let Some(line) = child.next_stdout_line().await.unwrap() {
		got.push(line);
	}
	child.finish().await.unwrap();
	got
}

/// Builds a command against an argv-echoing stub with copied args.
fn echo_cmd(series: BorgSeries, args: Vec<&str>) -> BorgCmd {
	let dir = borg_common::temp_dir(&format!("echo-{}", args.len()));
	let body = "#!/bin/sh\nfor a in \"$@\"; do echo \"$a\"; done\n";
	let bin = borg_common::stub_bor(&dir, "echo-borg", body);
	let mut cmd = BorgCmd::new(bin.to_str().unwrap(), series);
	for a in args {
		cmd = cmd.arg(a);
	}
	cmd
}

#[tokio::test]
async fn v1_repo_listing_args() {
	let cmd = echo_cmd(
		BorgSeries::V1_4,
		vec!["-r", "ssh://borg@gw.oldfire.de:22222/data/borg/repos/kayda"],
	)
	.repo_listing();
	assert_eq!(
		echo_argv(cmd).await,
		vec![
			"-r",
			"ssh://borg@gw.oldfire.de:22222/data/borg/repos/kayda",
			"list",
			"--json"
		]
	);
}

#[tokio::test]
async fn v2_repo_listing_uses_renamed_subcommand() {
	let cmd = echo_cmd(BorgSeries::V2_0B, vec!["-r", "/tmp/testrepo.borg"]).repo_listing();
	assert_eq!(
		echo_argv(cmd).await,
		vec!["-r", "/tmp/testrepo.borg", "repo-list", "--json"]
	);
}

#[tokio::test]
async fn v2_file_listing_with_aid_instance() {
	let cmd = echo_cmd(
		BorgSeries::V2_0B,
		vec!["-r", "/tmp/testrepo.borg", "list", "--json-lines"],
	)
	.archive("aid:9f1c");
	assert_eq!(
		echo_argv(cmd).await,
		vec!["-r", "/tmp/testrepo.borg", "list", "--json-lines", "aid:9f1c"]
	);
}
