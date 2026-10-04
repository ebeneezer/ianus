// SPDX-License-Identifier: GPL-3.0-or-later

//! Write helpers; statements are prepared per call (no caching) to keep
//! resident memory low.

use rusqlite::params;

use super::SqliteStore;
use crate::janus_kv::{Error, Key, Value};

impl SqliteStore {
	/// Upserts `value` at `key`.
	pub(crate) fn put_raw(&self, key: &Key, value: &Value) -> Result<(), Error> {
		let conn = self.conn.lock().unwrap_or_else(|e| e.into_inner());
		conn.execute(
			"INSERT INTO kv (key, value) VALUES (?1, ?2)
			 ON CONFLICT(key) DO UPDATE SET value = excluded.value",
			params![key.as_slice(), value],
		)?;
		Ok(())
	}

	/// Deletes `key`; returns `true` if a row was removed.
	pub(crate) fn del_raw(&self, key: &Key) -> Result<bool, Error> {
		let conn = self.conn.lock().unwrap_or_else(|e| e.into_inner());
		let n = conn.execute("DELETE FROM kv WHERE key = ?1", params![key.as_slice()])?;
		Ok(n > 0)
	}
}
