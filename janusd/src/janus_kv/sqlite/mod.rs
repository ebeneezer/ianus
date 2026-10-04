// SPDX-License-Identifier: GPL-3.0-or-later

//! SQLite reference adapter for the canonical KV API.

mod batch;
mod prefix;
mod read;
mod scan;
mod trait_impl;
mod write;

use std::path::Path;
use std::sync::Mutex;

use rusqlite::Connection;

use crate::janus_kv::Error;

/// SQLite-backed [`KvStore`](crate::janus_kv::store::KvStore) adapter.
pub struct SqliteStore {
	/// Serialized connection; rusqlite connections are not `Sync`.
	conn: Mutex<Connection>,
}

impl SqliteStore {
	/// Opens (or creates) the store at `path`; applies pragmas and schema.
	pub fn open(path: &Path) -> Result<Self, Error> {
		let conn = Connection::open(path)?;
		conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA synchronous=NORMAL;")?;
		conn.busy_timeout(std::time::Duration::from_millis(5000))?;
		conn.execute_batch("CREATE TABLE IF NOT EXISTS kv (key BLOB PRIMARY KEY, value BLOB NOT NULL)")?;
		Ok(SqliteStore { conn: Mutex::new(conn) })
	}
}
