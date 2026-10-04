// SPDX-License-Identifier: GPL-3.0-or-later

//! Repo objects behind the canonical KV API (DATAMODEL §1.1, §2.2).

pub mod error;
mod list;
mod model;
mod ops;
mod validate;

use std::sync::Arc;

use crate::janus_kv::KvStore;

pub use error::Error;
pub use list::RepoEntry;
pub use model::{RepoSpec, SCHEMA_VERSION};

/// Typed access to the `repo/` keyspace over the canonical KV API.
#[derive(Clone)]
pub struct RepoStore {
	kv: Arc<dyn KvStore>,
}

impl RepoStore {
	/// Wraps a KV store.
	pub fn new(kv: Arc<dyn KvStore>) -> Self {
		RepoStore { kv }
	}
}
