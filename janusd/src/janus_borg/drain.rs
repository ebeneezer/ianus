// SPDX-License-Identifier: GPL-3.0-or-later

//! Pipe draining for a running Borg child (stdout discarded, stderr tail kept).

use super::child::{BorgChild, read_line};
use super::error::Error;

impl BorgChild {
	/// Drains stdout (discarded) and stderr (only last line kept).
	pub(crate) async fn drain(&mut self) -> Result<(), Error> {
		loop {
			let out_open = self.drain_stdout().await?;
			let err_open = self.drain_stderr().await?;
			if !out_open && !err_open {
				return Ok(());
			}
		}
	}

	/// Drains one stdout line; returns whether the pipe is still open.
	async fn drain_stdout(&mut self) -> Result<bool, Error> {
		let Some(reader) = self.stdout.as_mut() else {
			return Ok(false);
		};
		Ok(read_line(reader).await?.is_some())
	}

	/// Drains one stderr line; returns whether the pipe is still open.
	async fn drain_stderr(&mut self) -> Result<bool, Error> {
		let Some(reader) = self.stderr.as_mut() else {
			return Ok(false);
		};
		let Some(line) = read_line(reader).await? else {
			return Ok(false);
		};
		if !line.trim().is_empty() {
			self.last_stderr = line.trim().to_string();
		}
		Ok(true)
	}
}
