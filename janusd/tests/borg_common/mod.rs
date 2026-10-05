// SPDX-License-Identifier: GPL-3.0-or-later

//! Shared helpers for borg stub tests (temp dirs, executable stubs).

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU32, Ordering};

static DIR_SEQ: AtomicU32 = AtomicU32::new(0);

/// Creates a unique temp dir for one test (counter avoids stub clashes).
pub fn temp_dir(name: &str) -> PathBuf {
	let seq = DIR_SEQ.fetch_add(1, Ordering::Relaxed);
	let dir = std::env::temp_dir().join(format!("janus-borg-test-{name}-{}-{seq}", std::process::id()));
	let _ = fs::remove_dir_all(&dir);
	fs::create_dir_all(&dir).expect("temp dir");
	dir
}

/// Writes an executable stub script with the given body.
pub fn stub_bor(dir: &Path, name: &str, body: &str) -> PathBuf {
	let path = dir.join(name);
	let mut f = fs::File::create(&path).expect("create stub");
	f.write_all(body.as_bytes()).expect("write stub");
	#[cfg(unix)]
	{
		use std::os::unix::fs::PermissionsExt;
		fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).expect("chmod");
	}
	path
}

/// Body of a stub that sleeps forever and exits 0 on TERM.
// Used by a subset of the test binaries linking this module.
#[allow(dead_code)]
pub fn sleep_stub_body() -> &'static str {
	"#!/bin/sh\necho ready\ntrap 'exit 0' TERM\nwhile true; do sleep 0.05; done\n"
}
