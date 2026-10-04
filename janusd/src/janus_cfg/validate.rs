// SPDX-License-Identifier: GPL-3.0-or-later

//! Validation of cfg values.

use serde_json::Value as Json;

use crate::janus_cfg::Error;

/// Schema versions accepted by this daemon.
pub const KNOWN_SCHEMA_VERSIONS: &[u64] = &[1];

/// Validates a cfg value: a JSON object with a known `schema_version`.
pub fn validate_value(v: &Json) -> Result<(), Error> {
	let obj = v
		.as_object()
		.ok_or_else(|| Error::InvalidValue("value must be a JSON object".to_string()))?;
	let version = obj
		.get("schema_version")
		.and_then(Json::as_u64)
		.ok_or_else(|| Error::InvalidValue("missing or non-integer schema_version".to_string()))?;
	if !KNOWN_SCHEMA_VERSIONS.contains(&version) {
		return Err(Error::InvalidValue(format!(
			"unsupported schema_version {version}; known: {KNOWN_SCHEMA_VERSIONS:?}"
		)));
	}
	Ok(())
}
