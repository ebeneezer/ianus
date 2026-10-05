// SPDX-License-Identifier: GPL-3.0-or-later

//! Errors for janus_borg subprocess control.

use thiserror::Error;

/// Errors raised while spawning, probing or running Borg subprocesses.
#[derive(Debug, Error)]
pub enum Error {
	/// Spawning the Borg binary failed.
	#[error("spawning borg failed: {0}")]
	Spawn(String),
	/// An I/O error occurred while reading or waiting.
	#[error(transparent)]
	Io(#[from] std::io::Error),
	/// The probed Borg version does not match a pinned series.
	#[error("borg version mismatch: found {found:?}, expected pin {expected:?}")]
	Version {
		/// Version string reported by `borg --version` (first line).
		found: String,
		/// Pin the series demanded (e.g. `1.4.` or `2.0.0b`).
		expected: &'static str,
	},
	/// The child exited with a non-zero status.
	#[error("borg exited with status {code}: {stderr}")]
	Exit {
		/// Exit code the child reported.
		code: i32,
		/// Last non-empty stderr line (bounded, best effort).
		stderr: String,
	},
}
