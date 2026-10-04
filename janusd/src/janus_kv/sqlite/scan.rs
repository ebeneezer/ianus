// SPDX-License-Identifier: GPL-3.0-or-later

//! Paged prefix scans.

use rusqlite::params;

use super::SqliteStore;
use super::prefix::prefix_upper;
use crate::janus_kv::{Error, Key, KvEntry};

impl SqliteStore {
	/// Ascending inclusive `prefix` scan; `after` exclusive, `limit` >= 1.
	pub(crate) fn scan_raw(&self, prefix: &Key, after: Option<&Key>, limit: u32) -> Result<Vec<KvEntry>, Error> {
		let upper = prefix_upper(prefix.as_slice());
		let after = after.map(|k| k.as_slice());
		let conn = self.conn.lock().unwrap_or_else(|e| e.into_inner());
		let mut stmt = conn.prepare(
			"SELECT key, value FROM kv
			 WHERE key >= ?1 AND (?2 IS NULL OR key < ?2)
			 AND (?3 IS NULL OR key > ?3) ORDER BY key ASC LIMIT ?4",
		)?;
		let mut rows = stmt.query(params![prefix.as_slice(), upper, after, limit])?;
		let mut out = Vec::new();
		while let Some(row) = rows.next()? {
			out.push(KvEntry {
				key: Key::from_bytes(row.get(0)?),
				value: row.get(1)?,
			});
		}
		Ok(out)
	}
}
