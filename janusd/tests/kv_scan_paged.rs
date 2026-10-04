// SPDX-License-Identifier: GPL-3.0-or-later

//! Paged prefix-scan tests with cursor continuation.

mod common;

use common::{key, store};
use janusd::janus_kv::{KvStore, Value};

#[tokio::test]
async fn paged_scan() {
	let store = store("scan-paged");
	for parts in [["repo", "1", "a"], ["repo", "1", "b"], ["repo", "1", "c"]] {
		store.put(key(&parts), Value::from(b"v".as_slice())).await.expect("put");
	}

	let prefix = key(&["repo", "1"]);
	let page1 = store.scan(&prefix, None, 2).await.expect("scan");
	assert_eq!(page1.len(), 2);
	assert_eq!(page1[0].key.as_slice(), b"repo/1/a");
	assert_eq!(page1[1].key.as_slice(), b"repo/1/b");

	let page2 = store.scan(&prefix, Some(&page1[1].key), 2).await.expect("scan");
	assert_eq!(page2.len(), 1);
	assert_eq!(page2[0].key.as_slice(), b"repo/1/c");

	let page3 = store.scan(&prefix, Some(&page2[0].key), 2).await.expect("scan");
	assert!(page3.is_empty());
}
