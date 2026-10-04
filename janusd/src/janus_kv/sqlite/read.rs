// SPDX-License-Identifier: GPL-3.0-or-later

//! Read helpers; statements are prepared per call (no caching) to keep
//! resident memory low.

use rusqlite::params;

use super::SqliteStore;
use crate::janus_kv::{Error, Key, Value};

impl SqliteStore {
	/// Reads `key`; returns `None` if absent.
	pub(crate) fn get_raw(&self, key: &Key) -> Result<Option<Value>, Error> {
		let conn = self.conn.lock().unwrap_or_else(|e| e.into_inner());
		let mut stmt = conn.prepare("SELECT value FROM kv WHERE key = ?1")?;
		let mut rows = stmt.query(params![key.as_slice()])?;
		match rows.next()? {
			Some(row) => Ok(Some(row.get(0)?)),
			None => Ok(None),
		}
	}

	/// Returns whether `key` exists.
	pub(crate) fn exists_raw(&self, key: &Key) -> Result<bool, Error> {
		let conn = self.conn.lock().unwrap_or_else(|e| e.into_inner());
		let mut stmt = conn.prepare("SELECT 1 FROM kv WHERE key = ?1")?;
		let mut rows = stmt.query(params![key.as_slice()])?;
		Ok(rows.next()?.is_some())
	}
}
