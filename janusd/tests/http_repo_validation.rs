// SPDX-License-Identifier: GPL-3.0-or-later

//! HTTP validation tests for the repo endpoints.

mod common;
mod http_common;

use axum::http::StatusCode;
use axum::response::Response;
use serde_json::{Value, json};

async fn put(name: &str, uri: &str, spec: &Value) -> Response {
	http_common::send(http_common::state(name), http_common::json_request("PUT", uri, spec)).await
}

#[tokio::test]
async fn unknown_field_is_422() {
	let spec = json!({"schema_version": 1, "name": "x", "location": "file:///tmp/repo", "extra": true});
	let resp = put("a", "/api/repos/x", &spec).await;
	assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn missing_location_is_422() {
	let spec = json!({"schema_version": 1, "name": "x"});
	let resp = put("b", "/api/repos/x", &spec).await;
	assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn bad_scheme_is_422() {
	let spec = json!({"schema_version": 1, "name": "x", "location": "http://x"});
	let resp = put("c", "/api/repos/x", &spec).await;
	assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);
	let body: Value = serde_json::from_slice(&http_common::body_bytes(resp).await).expect("json");
	let msg = body["error"].as_str().expect("error");
	assert!(msg.contains("location") && msg.contains("scheme"));
}

#[tokio::test]
async fn name_mismatch_is_422() {
	let spec = json!({"schema_version": 1, "name": "repo-b", "location": "file:///tmp/repo"});
	let resp = put("d", "/api/repos/repo-a", &spec).await;
	assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);
}
