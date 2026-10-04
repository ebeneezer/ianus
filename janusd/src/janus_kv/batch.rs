// SPDX-License-Identifier: GPL-3.0-or-later

//! Atomic write batches.

use crate::janus_kv::Value;
use crate::janus_kv::key::Key;

/// One operation of a [`KvStore::write_batch`](crate::janus_kv::store::KvStore::write_batch).
///
/// A batch applies atomically: all operations succeed or none take effect.
pub enum BatchOp {
	/// Insert or overwrite `value` at `key`.
	Put {
		/// Target key.
		key: Key,
		/// Value to store.
		value: Value,
	},
	/// Remove `key` if present.
	Delete {
		/// Target key.
		key: Key,
	},
}
