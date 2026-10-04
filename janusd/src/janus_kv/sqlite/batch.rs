// SPDX-License-Identifier: GPL-3.0-or-later

//! Atomic batch writes in a single transaction.

use rusqlite::params;

use crate::janus_kv::batch::BatchOp;
use crate::janus_kv::error::Error;
use crate::janus_kv::sqlite::SqliteStore;

impl SqliteStore {
	/// Applies `ops` atomically; rolls back on any error.
	pub(crate) fn batch_raw(&self, ops: &[BatchOp]) -> Result<(), Error> {
		let mut conn = self.conn.lock().unwrap_or_else(|e| e.into_inner());
		let tx = conn.transaction()?;
		for op in ops {
			match op {
				BatchOp::Put { key, value } => {
					tx.execute(
						"INSERT INTO kv (key, value) VALUES (?1, ?2)
						 ON CONFLICT(key) DO UPDATE SET value = excluded.value",
						params![key.as_slice(), value],
					)?;
				}
				BatchOp::Delete { key } => {
					tx.execute("DELETE FROM kv WHERE key = ?1", params![key.as_slice()])?;
				}
			}
		}
		tx.commit()?;
		Ok(())
	}
}
