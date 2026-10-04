// SPDX-License-Identifier: GPL-3.0-or-later

//! Listing of repo entries.

use serde::Serialize;

use super::{Error, RepoSpec, RepoStore};
use crate::janus_ident;
use crate::janus_kv::{Key, KvEntry};

/// One repository entry as returned by [`RepoStore::list`].
#[derive(Serialize)]
pub struct RepoEntry {
	/// Repository name without the `repo/` prefix.
	pub name: String,
	/// Parsed repo spec.
	pub value: RepoSpec,
}

impl RepoStore {
	/// Lists `repo/` entries; `after` is an exclusive cursor, `limit` is clamped to 1..=1000.
	pub async fn list(&self, after: Option<&str>, limit: u32) -> Result<Vec<RepoEntry>, Error> {
		if let Some(after) = after {
			janus_ident::validate_name(after).map_err(Error::InvalidName)?;
		}
		let limit = limit.clamp(1, 1000);
		let prefix = Key::from_parts(&["repo"])?;
		let after_key = after.map(|a| Key::from_parts(&["repo", a])).transpose()?;
		let entries = self.kv.scan(&prefix, after_key.as_ref(), limit).await?;
		entries.into_iter().map(repo_entry).collect()
	}
}

/// Converts one scan entry into a [`RepoEntry`].
fn repo_entry(entry: KvEntry) -> Result<RepoEntry, Error> {
	let name = entry
		.key
		.as_slice()
		.strip_prefix(b"repo/")
		.ok_or_else(|| Error::InvalidName("key outside repo/ namespace".to_string()))?;
	let name = String::from_utf8(name.to_vec()).map_err(|_| Error::InvalidName("non-UTF-8 repo key".to_string()))?;
	let value = RepoSpec::from_json(&entry.value)?;
	Ok(RepoEntry { name, value })
}
