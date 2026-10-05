// SPDX-License-Identifier: GPL-3.0-or-later

//! Cancel-ladder operations for a running Borg child (TASKFRAMEWORK §2).

use std::process::ExitStatus;

use nix::sys::signal::{Signal, kill};
use nix::unistd::Pid;
use tokio::time::timeout;

use super::child::BorgChild;
use super::error::Error;

impl BorgChild {
	/// Cancel ladder: SIGTERM, grace-drain, SIGKILL, reap (§2). Returns
	/// only after the child truly ended; safe after a prior finish.
	pub async fn cancel(&mut self) -> Result<ExitStatus, Error> {
		if self.done.is_none() {
			if let Ok(Some(status)) = self.child.try_wait() {
				self.done = Some(status);
			} else {
				self.signal(Signal::SIGTERM);
				if timeout(self.grace, self.drain()).await.is_err() {
					self.signal(Signal::SIGKILL);
				}
				self.done = Some(self.child.wait().await?);
			}
		}
		Ok(self.done.unwrap())
	}

	/// Sends a signal if the child still has a live pid.
	fn signal(&self, sig: Signal) {
		if let Some(pid) = self.child.id() {
			let _ = kill(Pid::from_raw(pid as i32), sig);
		}
	}
}
