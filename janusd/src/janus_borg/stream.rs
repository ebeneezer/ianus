// SPDX-License-Identifier: GPL-3.0-or-later

//! Bounded line streaming and finish semantics for a Borg child.

use std::process::ExitStatus;

use super::child::{BorgChild, read_line};
use super::error::Error;

impl BorgChild {
	/// Reads the next stdout line; `None` at EOF (bounded line reads).
	pub async fn next_stdout_line(&mut self) -> Result<Option<String>, Error> {
		match self.stdout.as_mut() {
			Some(reader) => read_line(reader).await,
			None => Ok(None),
		}
	}

	/// Last non-empty stderr line observed so far (bounded, best effort).
	pub fn last_stderr(&self) -> &str {
		&self.last_stderr
	}

	/// Drains both pipes and waits for exit (reap); non-zero yields [`Error::Exit`](Error::Exit).
	pub async fn finish(&mut self) -> Result<ExitStatus, Error> {
		self.drain().await?;
		let status = self.child.wait().await?;
		self.done = Some(status);
		self.status_result(status)
	}

	/// Maps an exit status to `Ok` (zero) or [`Error::Exit`](Error::Exit).
	pub(crate) fn status_result(&self, status: ExitStatus) -> Result<ExitStatus, Error> {
		match status.code() {
			Some(0) | None => Ok(status),
			Some(code) => Err(Error::Exit {
				code,
				stderr: self.last_stderr.clone(),
			}),
		}
	}
}
