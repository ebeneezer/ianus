// SPDX-License-Identifier: GPL-3.0-or-later

//! Error type shared by the KV store API and its adapters.

use thiserror::Error;

/// Errors returned by the KV store API.
#[derive(Debug, Error)]
pub enum Error {
	/// The caller supplied invalid input (e.g. a bad key part).
	#[error("invalid input: {0}")]
	InvalidInput(String),
	/// The SQLite backend failed.
	#[error("sqlite backend: {0}")]
	Sqlite(#[from] rusqlite::Error),
}
