// SPDX-License-Identifier: GPL-3.0-or-later

//! Validation of repo specs.

use super::{Error, RepoSpec, SCHEMA_VERSION};
use crate::janus_ident;

/// Accepted location scheme prefixes (lowercase).
const LOCATION_SCHEMES: &[&str] = &["file://", "ssh://", "sftp://", "borg://"];

impl RepoSpec {
	/// Validates all fields; returns [`Error::InvalidValue`] on failure.
	pub fn validate(&self) -> Result<(), Error> {
		if self.schema_version != SCHEMA_VERSION {
			return Err(Error::InvalidValue(format!(
				"unsupported schema_version {}; known: {SCHEMA_VERSION}",
				self.schema_version
			)));
		}
		janus_ident::validate_name(&self.name).map_err(Error::InvalidName)?;
		validate_location(&self.location)?;
		if let Some(secret_ref) = &self.secret_ref {
			validate_secret_ref(secret_ref)?;
		}
		if let Some(label) = &self.label
			&& label.len() > 256
		{
			return Err(Error::InvalidValue("label must be at most 256 chars".to_string()));
		}
		if let Some(notes) = &self.notes
			&& notes.len() > 4096
		{
			return Err(Error::InvalidValue("notes must be at most 4096 chars".to_string()));
		}
		Ok(())
	}
}

/// Validates a location: non-empty, ≤2048 chars, known scheme.
fn validate_location(location: &str) -> Result<(), Error> {
	if location.is_empty() || location.len() > 2048 {
		return Err(Error::InvalidValue(
			"location must be non-empty and at most 2048 chars".to_string(),
		));
	}
	let lower = location.to_ascii_lowercase();
	if !LOCATION_SCHEMES.iter().any(|s| lower.starts_with(s)) {
		return Err(Error::InvalidValue(format!(
			"invalid location scheme: expected one of {LOCATION_SCHEMES:?}"
		)));
	}
	Ok(())
}

/// Validates a secret reference: non-empty, no `/`, ≤128 chars.
fn validate_secret_ref(secret_ref: &str) -> Result<(), Error> {
	if secret_ref.is_empty() || secret_ref.contains('/') || secret_ref.len() > 128 {
		return Err(Error::InvalidValue(
			"invalid secret_ref: non-empty, no '/', at most 128 chars".to_string(),
		));
	}
	Ok(())
}
