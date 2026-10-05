// SPDX-License-Identifier: GPL-3.0-or-later

//! Command builder for pinned Borg series (series-dispatched listing).

use std::process::Stdio;

use tokio::process::Command;

use super::version::BorgSeries;

/// Table: repo-level listing subcommand per series (ARCHITECTURE §3).
const REPO_LIST_SUBCMD: [&str; 2] = ["list", "repo-list"];

/// Builds `borg` argument vectors per pinned series.
#[derive(Debug, Clone)]
pub struct BorgCmd {
	/// Borg binary to execute.
	pub bin: String,
	/// Series pin the command is built for.
	pub series: BorgSeries,
	/// Collected arguments (built in order).
	pub args: Vec<String>,
}

impl BorgCmd {
	/// Starts a command for the given binary and pinned series.
	pub fn new(bin: &str, series: BorgSeries) -> Self {
		BorgCmd {
			bin: bin.to_string(),
			series,
			args: Vec::new(),
		}
	}

	/// Appends `-r <location>` (unified V1+V2 repo addressing).
	pub fn repo(mut self, location: &str) -> Self {
		self.args.extend(["-r".into(), location.into()]);
		self
	}

	/// Appends a positional archive (V2 also accepts `aid:` instances).
	pub fn archive(mut self, archive: &str) -> Self {
		self.args.push(archive.into());
		self
	}

	/// Appends a bare argument (e.g. `--json-lines`).
	pub fn arg(mut self, a: &str) -> Self {
		self.args.push(a.into());
		self
	}

	/// Appends repo-level listing: `list --json` (V1) / `repo-list --json` (V2).
	pub fn repo_listing(mut self) -> Self {
		let idx = usize::from(self.series == BorgSeries::V2_0B);
		self.args.push(REPO_LIST_SUBCMD[idx].into());
		self.args.push("--json".into());
		self
	}

	/// Appends file-level listing arguments (`list --json-lines`).
	pub fn file_listing(self) -> Self {
		self.arg("list").arg("--json-lines")
	}

	/// Spawns the built command with piped stdout/stderr and null stdin.
	pub fn build(self) -> Command {
		let mut cmd = Command::new(&self.bin);
		cmd.args(&self.args)
			.stdin(Stdio::null())
			.stdout(Stdio::piped())
			.stderr(Stdio::piped());
		cmd
	}
}
