// SPDX-License-Identifier: GPL-3.0-or-later

//! Inclusive prefix-bound regression test (prefix-equal entry).

mod common;

use common::{key, store};
use janusd::janus_kv::{KvStore, Value};

#[tokio::test]
async fn prefix_entry_is_inclusive() {
	let store = store("scan-inclusive");
	let k = key(&["repo", "1"]);
	store.put(k, Value::from(b"meta".as_slice())).await.expect("put");
	let k = key(&["repo", "1", "a"]);
	store.put(k, Value::from(b"v".as_slice())).await.expect("put");
	let page = store.scan(&key(&["repo", "1"]), None, 10).await.expect("scan");
	assert_eq!(page.len(), 2);
	assert_eq!(page[0].key.as_slice(), b"repo/1");
	assert_eq!(page[1].key.as_slice(), b"repo/1/a");
}
