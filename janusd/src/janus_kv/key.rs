// SPDX-License-Identifier: GPL-3.0-or-later

//! Hierarchical keys over ASCII parts (DATAMODEL §2.2).

use crate::janus_kv::Error;

/// A hierarchical byte key, e.g. `repo/<id>` or `cfg/<key>`.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Key(Vec<u8>);

impl Key {
	/// Wraps raw bytes as a key.
	pub fn from_bytes(bytes: Vec<u8>) -> Self {
		Key(bytes)
	}

	/// Joins ASCII parts with `/`; rejects empty parts and embedded `/`.
	pub fn from_parts(parts: &[&str]) -> Result<Self, Error> {
		let mut out = Vec::new();
		for (i, part) in parts.iter().enumerate() {
			if part.is_empty() || part.contains('/') {
				return Err(Error::InvalidInput(format!("bad key part: {part:?}")));
			}
			if i > 0 {
				out.push(b'/');
			}
			out.extend_from_slice(part.as_bytes());
		}
		Ok(Key(out))
	}

	/// Returns the key bytes.
	pub fn as_slice(&self) -> &[u8] {
		&self.0
	}

	/// Consumes the key, returning its bytes.
	pub fn into_inner(self) -> Vec<u8> {
		self.0
	}
}
