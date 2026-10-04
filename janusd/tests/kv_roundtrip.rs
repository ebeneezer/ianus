// SPDX-License-Identifier: GPL-3.0-or-later

//! Round-trip tests for the SQLite adapter.

mod common;

use common::{key, store};
use janusd::janus_kv::{KvStore, Value};

#[tokio::test]
async fn roundtrip() {
	let store = store("roundtrip");
	let key = key(&["cfg", "theme"]);
	assert_eq!(key.as_slice(), b"cfg/theme");

	assert_eq!(store.get(&key).await.expect("get"), None);
	store
		.put(key.clone(), Value::from(b"dark".as_slice()))
		.await
		.expect("put");
	assert_eq!(
		store.get(&key).await.expect("get"),
		Some(Value::from(b"dark".as_slice()))
	);
	store
		.put(key.clone(), Value::from(b"light".as_slice()))
		.await
		.expect("overwrite");
	assert_eq!(
		store.get(&key).await.expect("get"),
		Some(Value::from(b"light".as_slice()))
	);
	assert!(store.exists(&key).await.expect("exists"));
	assert!(store.delete(&key).await.expect("delete"));
	assert_eq!(store.get(&key).await.expect("get"), None);
	assert!(!store.delete(&key).await.expect("delete again"));
}
