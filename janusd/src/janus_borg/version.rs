// SPDX-License-Identifier: GPL-3.0-or-later

//! Pinned Borg series and version probing (DATAMODEL §3.1, ARCHITECTURE §3).

/// A supported, pinned Borg series.
///
/// `V1_4` covers Borg 1.4.x (client and maintainer server 1.4.0),
/// `V2_0B` covers the Borg 2.0.0b series (verified at 2.0.0b25).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BorgSeries {
	/// Borg 1.4.x (classic `list --json`/`--json-lines`).
	V1_4,
	/// Borg 2.0.0b series (`repo-list --json`, `path_b64`).
	V2_0B,
}

impl BorgSeries {
	/// Version prefix accepted for this series (probe pin).
	pub fn pin(self) -> &'static str {
		match self {
			BorgSeries::V1_4 => "1.4.",
			BorgSeries::V2_0B => "2.0.0b",
		}
	}
}

/// Maps a `borg --version` output like `borg 2.0.0b25` to a pinned series.
///
/// Returns `None` for anything that matches neither pin; callers must
/// treat that as unsupported and refuse to run against it.
pub fn series_from_version_string(output: &str) -> Option<BorgSeries> {
	let ver = output.trim().lines().next()?.strip_prefix("borg ")?;
	if ver.starts_with(BorgSeries::V1_4.pin()) {
		Some(BorgSeries::V1_4)
	} else if ver.starts_with(BorgSeries::V2_0B.pin()) {
		Some(BorgSeries::V2_0B)
	} else {
		None
	}
}

/// Validates the version output of one probed binary against a demanded pin.
///
/// `expected` is the pin of the demanded series; `found` is the full first
/// line of `borg --version`. Fails with [`Error::Version`](crate::janus_borg::Error::Version).
pub fn check_version(demand: BorgSeries, found: &str) -> Result<String, crate::janus_borg::Error> {
	let line = found.trim().lines().next().unwrap_or_default();
	if series_from_version_string(found) == Some(demand) {
		return Ok(line.to_string());
	}
	Err(crate::janus_borg::Error::Version {
		found: line.to_string(),
		expected: demand.pin(),
	})
}
