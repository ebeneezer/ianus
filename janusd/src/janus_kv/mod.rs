// SPDX-License-Identifier: GPL-3.0-or-later

//! Canonical KV store: keys, values, scan pages, batches, and the
//! [`KvStore`] trait with its SQLite reference adapter.

pub mod batch;
pub mod error;
pub mod key;
pub mod scan;
pub mod sqlite;
pub mod store;

pub use batch::BatchOp;
pub use error::Error;
pub use key::Key;
pub use scan::KvEntry;
pub use sqlite::SqliteStore;
pub use store::KvStore;

/// A stored value: raw bytes (JSON objects per DATAMODEL §2.1).
pub type Value = Vec<u8>;
