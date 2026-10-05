// SPDX-License-Identifier: GPL-3.0-or-later

//! Child process handle: spawn and lifecycle state.

use std::process::ExitStatus;
use std::time::Duration;

use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Child, ChildStderr, ChildStdout, Command};

use super::error::Error;

/// Default grace period between SIGTERM and SIGKILL.
pub const DEFAULT_GRACE: Duration = Duration::from_secs(5);

/// A running Borg child with bounded line streaming and cancel support.
pub struct BorgChild {
	/// The underlying tokio child process (pipes already taken).
	pub(crate) child: Child,
	/// Buffered stdout reader (persistent across line reads).
	pub(crate) stdout: Option<BufReader<ChildStdout>>,
	/// Buffered stderr reader (persistent across line reads).
	pub(crate) stderr: Option<BufReader<ChildStderr>>,
	/// Grace before SIGKILL (cancel ladder).
	pub(crate) grace: Duration,
	/// Last non-empty stderr line observed (bounded, best effort).
	pub(crate) last_stderr: String,
	/// Cached exit status once the child truly ended.
	pub(crate) done: Option<ExitStatus>,
}

/// Reads the next line of a buffered pipe reader; `None` at EOF.
pub(crate) async fn read_line<R: tokio::io::AsyncBufRead + Unpin>(reader: &mut R) -> Result<Option<String>, Error> {
	let mut line = String::new();
	let n = reader.read_line(&mut line).await?;
	Ok(if n == 0 {
		None
	} else {
		Some(line.trim_end().to_string())
	})
}

impl BorgChild {
	/// Spawns a built command; pipes are taken into buffered readers.
	pub fn spawn(mut cmd: Command) -> Result<Self, Error> {
		let mut child = cmd.spawn().map_err(|e| Error::Spawn(e.to_string()))?;
		let stdout = child.stdout.take().map(BufReader::new);
		let stderr = child.stderr.take().map(BufReader::new);
		Ok(BorgChild {
			child,
			stdout,
			stderr,
			grace: DEFAULT_GRACE,
			last_stderr: String::new(),
			done: None,
		})
	}

	/// Overrides the grace period used before SIGKILL.
	pub fn set_grace(&mut self, grace: Duration) {
		self.grace = grace;
	}
}
