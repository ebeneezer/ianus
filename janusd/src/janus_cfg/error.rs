// SPDX-License-Identifier: GPL-3.0-or-later

//! Error type for the cfg module.

use thiserror::Error;

/// Errors returned by the cfg module.
#[derive(Debug, Error)]
pub enum Error {
	/// The cfg name is invalid.
	#[error("invalid cfg name: {0}")]
	InvalidName(String),
	/// The cfg value is invalid.
	#[error("invalid cfg value: {0}")]
	InvalidValue(String),
	/// The underlying KV store failed.
	#[error("kv backend: {0}")]
	Kv(#[from] crate::janus_kv::Error),
	/// JSON serialization or parsing failed.
	#[error("json: {0}")]
	Json(#[from] serde_json::Error),
}
