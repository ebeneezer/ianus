// SPDX-License-Identifier: GPL-3.0-or-later

//! HTTP tests for the repo list endpoint.

mod common;
mod http_common;

use axum::http::StatusCode;
use serde_json::{Value, json};

#[tokio::test]
async fn limit_zero_is_400() {
	let resp = http_common::send(
		http_common::state("a"),
		http_common::empty_request("GET", "/api/repos?limit=0"),
	)
	.await;
	assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn list_returns_stored_entry() {
	let state = http_common::state("b");
	let spec = json!({"schema_version": 1, "name": "ok", "location": "file:///tmp/repo"});
	let resp = http_common::send(state.clone(), http_common::json_request("PUT", "/api/repos/ok", &spec)).await;
	assert_eq!(resp.status(), StatusCode::NO_CONTENT);
	let resp = http_common::send(state, http_common::empty_request("GET", "/api/repos?limit=5")).await;
	assert_eq!(resp.status(), StatusCode::OK);
	let entries: Vec<Value> = serde_json::from_slice(&http_common::body_bytes(resp).await).expect("json");
	assert!(entries.iter().any(|e| e["name"] == "ok"));
}
