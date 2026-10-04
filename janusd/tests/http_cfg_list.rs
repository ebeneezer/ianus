// SPDX-License-Identifier: GPL-3.0-or-later

//! HTTP tests for the cfg list endpoint.

mod common;
mod http_common;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use serde_json::{Value, json};

#[tokio::test]
async fn limit_zero_is_400() {
	let state = http_common::state("http-val-e");
	let value = json!({ "schema_version": 1 });
	let resp = http_common::send(state.clone(), http_common::json_request("PUT", "/api/cfg/ok", &value)).await;
	assert_eq!(resp.status(), StatusCode::NO_CONTENT);
	let req = Request::builder()
		.uri("/api/cfg?limit=0")
		.body(Body::empty())
		.expect("request");
	let resp = http_common::send(state, req).await;
	assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn list_returns_stored_entry() {
	let state = http_common::state("http-val-f");
	let value = json!({ "schema_version": 1 });
	let resp = http_common::send(state.clone(), http_common::json_request("PUT", "/api/cfg/ok", &value)).await;
	assert_eq!(resp.status(), StatusCode::NO_CONTENT);
	let req = Request::builder().uri("/api/cfg").body(Body::empty()).expect("request");
	let resp = http_common::send(state, req).await;
	assert_eq!(resp.status(), StatusCode::OK);
	let entries: Vec<Value> = serde_json::from_slice(&http_common::body_bytes(resp).await).expect("json");
	assert!(entries.iter().any(|e| e["name"] == "ok"));
}
