// SPDX-License-Identifier: GPL-3.0-or-later

//! Atomic write_batch application tests.

mod common;

use common::{key, store};
use janusd::janus_kv::{BatchOp, KvStore, Value};

#[tokio::test]
async fn batch_puts_visible_together() {
	let store = store("batch");
	let ops = vec![
		BatchOp::Put {
			key: key(&["repo", "1", "a"]),
			value: Value::from(b"1".as_slice()),
		},
		BatchOp::Put {
			key: key(&["repo", "1", "b"]),
			value: Value::from(b"2".as_slice()),
		},
		BatchOp::Put {
			key: key(&["repo", "1", "c"]),
			value: Value::from(b"3".as_slice()),
		},
	];
	store.write_batch(ops).await.expect("batch");
	let page = store.scan(&key(&["repo", "1"]), None, 10).await.expect("scan");
	assert_eq!(page.len(), 3);
}

#[tokio::test]
async fn empty_batch_is_noop() {
	let store = store("batch-empty");
	store.write_batch(vec![]).await.expect("empty batch");
}
