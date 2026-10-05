// SPDX-License-Identifier: GPL-3.0-or-later

//! Repo object model (DATAMODEL §1.1, §2.1).

use serde::{Deserialize, Serialize};

use crate::janus_repo::Error;

/// Schema version of repo objects written by this daemon.
pub const SCHEMA_VERSION: u64 = 1;

/// A repository object stored at `repo/<name>`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RepoSpec {
	/// Object schema version; must equal [`SCHEMA_VERSION`].
	pub schema_version: u64,
	/// Repository name; must equal the key name.
	pub name: String,
	/// Borg repository location (`file://`, `ssh://`, `sftp://`, `borg://`).
	pub location: String,
	/// Optional reference to a secret (passphrase/key) by name.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub secret_ref: Option<String>,
	/// Optional human-readable label.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub label: Option<String>,
	/// Optional free-form notes.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub notes: Option<String>,
}

impl RepoSpec {
	/// Creates a minimal spec with the given name and location.
	pub fn new(name: &str, location: &str) -> Self {
		RepoSpec {
			schema_version: SCHEMA_VERSION,
			name: name.to_string(),
			location: location.to_string(),
			secret_ref: None,
			label: None,
			notes: None,
		}
	}

	/// Parses and validates a spec from JSON bytes.
	pub fn from_json(bytes: &[u8]) -> Result<Self, Error> {
		let spec: RepoSpec = serde_json::from_slice(bytes).map_err(|e| Error::InvalidValue(e.to_string()))?;
		spec.validate()?;
		Ok(spec)
	}

	/// Serializes the spec to JSON bytes.
	pub fn to_json(&self) -> Result<Vec<u8>, Error> {
		Ok(serde_json::to_vec(self)?)
	}
}
