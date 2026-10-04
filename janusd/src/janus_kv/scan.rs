// SPDX-License-Identifier: GPL-3.0-or-later

//! Scan results and page semantics.

use crate::janus_kv::Value;
use crate::janus_kv::key::Key;

/// One key/value pair of a scan page.
///
/// Each page is read consistently; there is no cross-page snapshot
/// guarantee — concurrent writes may appear or disappear between pages.
/// A strict snapshot cursor may be added with the tree-build tranche.
pub struct KvEntry {
	/// The entry key.
	pub key: Key,
	/// The entry value.
	pub value: Value,
}
