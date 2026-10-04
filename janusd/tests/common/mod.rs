// SPDX-License-Identifier: GPL-3.0-or-later

//! Shared helpers for integration tests.

use std::path::PathBuf;

use janusd::janus_kv::{Key, SqliteStore};

/// Path of a fresh SQLite database for `name` (unique per test process).
pub fn temp_db(name: &str) -> PathBuf {
	let dir = std::env::temp_dir().join(format!("janus-{name}-{}", std::process::id()));
	std::fs::create_dir_all(&dir).expect("create temp dir");
	dir.join("kv.db")
}

/// Builds a key from ASCII parts; panics on invalid input (tests only).
#[allow(dead_code)]
pub fn key(parts: &[&str]) -> Key {
	Key::from_parts(parts).expect("valid key")
}

/// Opens the store under test at a fresh temp path for `name`.
pub fn store(name: &str) -> SqliteStore {
	SqliteStore::open(&temp_db(name)).expect("open store")
}
