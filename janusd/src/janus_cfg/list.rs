// SPDX-License-Identifier: GPL-3.0-or-later

//! Listing of cfg entries.

use serde::Serialize;
use serde_json::Value as Json;

use super::{CfgStore, Error, validate_name};
use crate::janus_kv::{Key, KvEntry};

/// One configuration entry as returned by [`CfgStore::list`].
#[derive(Serialize)]
pub struct CfgEntry {
	/// Entry name without the `cfg/` prefix.
	pub name: String,
	/// Parsed JSON value.
	pub value: Json,
}

impl CfgStore {
	/// Lists `cfg/` entries; `after` is an exclusive cursor, `limit` is clamped to 1..=1000.
	pub async fn list(&self, after: Option<&str>, limit: u32) -> Result<Vec<CfgEntry>, Error> {
		if let Some(after) = after {
			validate_name(after)?;
		}
		let limit = limit.clamp(1, 1000);
		let prefix = Key::from_parts(&["cfg"])?;
		let after_key = after.map(|a| Key::from_parts(&["cfg", a])).transpose()?;
		let entries = self.kv.scan(&prefix, after_key.as_ref(), limit).await?;
		entries.into_iter().map(cfg_entry).collect()
	}
}

/// Converts one scan entry into a [`CfgEntry`].
fn cfg_entry(entry: KvEntry) -> Result<CfgEntry, Error> {
	let name = entry
		.key
		.as_slice()
		.strip_prefix(b"cfg/")
		.ok_or_else(|| Error::InvalidName("key outside cfg/ namespace".to_string()))?;
	let name = String::from_utf8(name.to_vec()).map_err(|_| Error::InvalidName("non-UTF-8 cfg key".to_string()))?;
	let value = serde_json::from_slice(&entry.value)?;
	Ok(CfgEntry { name, value })
}
