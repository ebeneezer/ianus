// SPDX-License-Identifier: GPL-3.0-or-later

//! Single-entry read/write operations of the cfg store.

use serde_json::Value as Json;

use super::{CfgStore, Error, validate, validate_name};
use crate::janus_kv::Key;

impl CfgStore {
	/// Returns the value at `cfg/<name>`, or `None` if absent.
	pub async fn get(&self, name: &str) -> Result<Option<Json>, Error> {
		validate_name(name)?;
		let key = Key::from_parts(&["cfg", name])?;
		match self.kv.get(&key).await? {
			None => Ok(None),
			Some(bytes) => Ok(Some(serde_json::from_slice(&bytes)?)),
		}
	}

	/// Validates and stores `value` at `cfg/<name>`.
	pub async fn put(&self, name: &str, value: &Json) -> Result<(), Error> {
		validate_name(name)?;
		validate::validate_value(value)?;
		let key = Key::from_parts(&["cfg", name])?;
		self.kv.put(key, serde_json::to_vec(value)?).await?;
		Ok(())
	}
}
