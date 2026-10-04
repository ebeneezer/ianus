// SPDX-License-Identifier: GPL-3.0-or-later

//! `KvStore` implementation delegating to the raw helpers.

use async_trait::async_trait;

use super::SqliteStore;
use crate::janus_kv::{BatchOp, Error, Key, KvEntry, KvStore, Value};

#[async_trait]
impl KvStore for SqliteStore {
	async fn get(&self, key: &Key) -> Result<Option<Value>, Error> {
		self.get_raw(key)
	}

	async fn put(&self, key: Key, value: Value) -> Result<(), Error> {
		self.put_raw(&key, &value)
	}

	async fn delete(&self, key: &Key) -> Result<bool, Error> {
		self.del_raw(key)
	}

	async fn exists(&self, key: &Key) -> Result<bool, Error> {
		self.exists_raw(key)
	}

	async fn scan(&self, prefix: &Key, after: Option<&Key>, limit: u32) -> Result<Vec<KvEntry>, Error> {
		self.scan_raw(prefix, after, limit)
	}

	async fn write_batch(&self, ops: Vec<BatchOp>) -> Result<(), Error> {
		self.batch_raw(&ops)
	}
}
