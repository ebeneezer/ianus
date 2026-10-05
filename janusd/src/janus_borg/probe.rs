// SPDX-License-Identifier: GPL-3.0-or-later

//! Version probing against a pinned Borg series.

use std::process::Stdio;

use tokio::process::Command;

use super::child::BorgChild;
use super::error::Error;
use super::version::{BorgSeries, check_version};

/// Probes `borg --version` and validates it against a demanded pin.
///
/// Output is tiny (one line); it is collected via bounded line reads.
pub async fn probe_version(bin: &str, demand: BorgSeries) -> Result<String, Error> {
	let mut cmd = Command::new(bin);
	cmd.arg("--version")
		.stdin(Stdio::null())
		.stdout(Stdio::piped())
		.stderr(Stdio::piped());
	let mut child = BorgChild::spawn(cmd)?;
	let mut out = String::new();
	while let Some(line) = child.next_stdout_line().await? {
		out.push_str(&line);
		out.push('\n');
	}
	child.finish().await?;
	check_version(demand, &out)
}
