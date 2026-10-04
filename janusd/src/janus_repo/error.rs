// SPDX-License-Identifier: GPL-3.0-or-later

//! Error type for the repo module.

use thiserror::Error;

/// Errors returned by the repo module.
#[derive(Debug, Error)]
pub enum Error {
	/// The repo name is invalid.
	#[error("invalid repo name: {0}")]
	InvalidName(String),
	/// The repo spec is invalid.
	#[error("invalid repo value: {0}")]
	InvalidValue(String),
	/// The underlying KV store failed.
	#[error("kv backend: {0}")]
	Kv(#[from] crate::janus_kv::Error),
	/// JSON serialization or parsing failed.
	#[error("json: {0}")]
	Json(#[from] serde_json::Error),
}
