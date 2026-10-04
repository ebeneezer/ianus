// SPDX-License-Identifier: GPL-3.0-or-later

//! HTTP round-trip tests for the health and cfg endpoints.

mod common;
mod http_common;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use serde_json::{Value, json};

#[tokio::test]
async fn health_ok() {
	let req = Request::builder()
		.uri("/api/health")
		.body(Body::empty())
		.expect("request");
	let resp = http_common::send(http_common::state("http-rt-health"), req).await;
	assert_eq!(resp.status(), StatusCode::OK);
	let body: Value = serde_json::from_slice(&http_common::body_bytes(resp).await).expect("json");
	assert_eq!(body, json!({ "status": "ok" }));
}

#[tokio::test]
async fn put_then_get_roundtrip() {
	let state = http_common::state("http-rt-put");
	let value = json!({ "schema_version": 1, "name": "janus-teal" });
	let resp = http_common::send(
		state.clone(),
		http_common::json_request("PUT", "/api/cfg/theme", &value),
	)
	.await;
	assert_eq!(resp.status(), StatusCode::NO_CONTENT);

	let req = Request::builder()
		.uri("/api/cfg/theme")
		.body(Body::empty())
		.expect("request");
	let resp = http_common::send(state, req).await;
	assert_eq!(resp.status(), StatusCode::OK);
	let body: Value = serde_json::from_slice(&http_common::body_bytes(resp).await).expect("json");
	assert_eq!(body, value);
}

#[tokio::test]
async fn get_missing_returns_404() {
	let req = Request::builder()
		.uri("/api/cfg/missing")
		.body(Body::empty())
		.expect("request");
	let resp = http_common::send(http_common::state("http-rt-missing"), req).await;
	assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}
