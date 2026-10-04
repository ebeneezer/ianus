// SPDX-License-Identifier: GPL-3.0-or-later

//! Mixed batch (put + delete) tests.

mod common;

use common::{key, store};
use janusd::janus_kv::{BatchOp, KvStore, Value};

#[tokio::test]
async fn batch_delete_and_puts() {
	let store = store("batch-mixed");
	store
		.put(key(&["repo", "1", "a"]), Value::from(b"old".as_slice()))
		.await
		.expect("put");
	let ops = vec![
		BatchOp::Put {
			key: key(&["repo", "1", "b"]),
			value: Value::from(b"2".as_slice()),
		},
		BatchOp::Delete {
			key: key(&["repo", "1", "a"]),
		},
	];
	store.write_batch(ops).await.expect("batch");
	let page = store.scan(&key(&["repo", "1"]), None, 10).await.expect("scan");
	assert_eq!(page.len(), 1);
	assert_eq!(page[0].key.as_slice(), b"repo/1/b");
}
