// SPDX-License-Identifier: GPL-3.0-or-later

//! Shared identifier validation (CODESTYLE §1.2 code reuse).

/// Validates a Janus identifier: non-empty, no `/`, at most 128 chars.
///
/// Returns the offending name on failure.
pub fn validate_name(name: &str) -> Result<(), String> {
	if name.is_empty() || name.contains('/') || name.len() > 128 {
		return Err(name.to_string());
	}
	Ok(())
}
