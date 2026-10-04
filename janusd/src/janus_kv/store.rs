// SPDX-License-Identifier: GPL-3.0-or-later

//! The canonical KV access API (ARCHITECTURE §5).

use async_trait::async_trait;

use crate::janus_kv::{BatchOp, Error, Key, KvEntry, Value};

/// Canonical key/value access; all modules use only this API.
#[async_trait]
pub trait KvStore: Send + Sync {
	/// Value at `key`, or `None` if absent.
	async fn get(&self, key: &Key) -> Result<Option<Value>, Error>;

	/// Upserts `value` at `key`.
	async fn put(&self, key: Key, value: Value) -> Result<(), Error>;

	/// Deletes `key`; `true` if it existed.
	async fn delete(&self, key: &Key) -> Result<bool, Error>;

	/// Whether `key` exists.
	async fn exists(&self, key: &Key) -> Result<bool, Error>;

	/// Ascending inclusive `prefix` scan; `after` exclusive, `limit` >= 1.
	async fn scan(&self, prefix: &Key, after: Option<&Key>, limit: u32) -> Result<Vec<KvEntry>, Error>;

	/// Applies `ops` atomically.
	async fn write_batch(&self, ops: Vec<BatchOp>) -> Result<(), Error>;
}
