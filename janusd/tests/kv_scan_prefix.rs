// SPDX-License-Identifier: GPL-3.0-or-later

//! Prefix isolation tests.

mod common;

use common::{key, store};
use janusd::janus_kv::{KvStore, Value};

#[tokio::test]
async fn prefix_isolation() {
	let store = store("scan-prefix");
	let parts: [&[&str]; 3] = [&["repo", "1", "a"], &["repo", "2", "a"], &["cfg", "x"]];
	for p in parts {
		store.put(key(p), Value::from(b"v".as_slice())).await.expect("put");
	}
	let repo1 = store.scan(&key(&["repo", "1"]), None, 10).await.expect("scan");
	assert_eq!(repo1.len(), 1);
	assert_eq!(repo1[0].key.as_slice(), b"repo/1/a");
	let repo2 = store.scan(&key(&["repo", "2"]), None, 10).await.expect("scan");
	assert_eq!(repo2.len(), 1);
	assert_eq!(repo2[0].key.as_slice(), b"repo/2/a");
}
