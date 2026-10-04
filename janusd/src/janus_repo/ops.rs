// SPDX-License-Identifier: GPL-3.0-or-later

//! Single-entry read/write operations of the repo store.

use super::{Error, RepoSpec, RepoStore};
use crate::janus_ident;
use crate::janus_kv::Key;

impl RepoStore {
	/// Returns the spec at `repo/<name>`, or `None` if absent.
	pub async fn get(&self, name: &str) -> Result<Option<RepoSpec>, Error> {
		janus_ident::validate_name(name).map_err(Error::InvalidName)?;
		let key = Key::from_parts(&["repo", name])?;
		match self.kv.get(&key).await? {
			None => Ok(None),
			Some(bytes) => Ok(Some(RepoSpec::from_json(&bytes)?)),
		}
	}

	/// Validates and stores `spec` at `repo/<name>` (upsert).
	pub async fn put(&self, name: &str, spec: &RepoSpec) -> Result<(), Error> {
		janus_ident::validate_name(name).map_err(Error::InvalidName)?;
		if spec.name != name {
			return Err(Error::InvalidValue(format!(
				"name mismatch: path name {name:?} != body name {:?}",
				spec.name
			)));
		}
		spec.validate()?;
		let key = Key::from_parts(&["repo", name])?;
		self.kv.put(key, spec.to_json()?).await?;
		Ok(())
	}

	/// Deletes `repo/<name>`; `false` if it was absent.
	pub async fn delete(&self, name: &str) -> Result<bool, Error> {
		janus_ident::validate_name(name).map_err(Error::InvalidName)?;
		let key = Key::from_parts(&["repo", name])?;
		Ok(self.kv.delete(&key).await?)
	}
}
