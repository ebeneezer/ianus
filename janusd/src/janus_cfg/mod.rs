// SPDX-License-Identifier: GPL-3.0-or-later

//! Janus self-configuration behind the canonical KV API (DATAMODEL §2.5).

pub mod error;
mod list;
mod ops;
pub mod validate;

use std::sync::Arc;

use crate::janus_ident;
use crate::janus_kv::KvStore;

pub use error::Error;
pub use list::CfgEntry;

/// Typed access to the `cfg/` keyspace over the canonical KV API.
#[derive(Clone)]
pub struct CfgStore {
	kv: Arc<dyn KvStore>,
}

impl CfgStore {
	/// Wraps a KV store.
	pub fn new(kv: Arc<dyn KvStore>) -> Self {
		CfgStore { kv }
	}
}

/// Validates a cfg name: non-empty, no `/`, at most 128 chars.
pub fn validate_name(name: &str) -> Result<(), Error> {
	janus_ident::validate_name(name).map_err(Error::InvalidName)
}
