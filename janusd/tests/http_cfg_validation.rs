// SPDX-License-Identifier: GPL-3.0-or-later

//! HTTP validation tests for the cfg endpoints.

mod common;
mod http_common;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use serde_json::{Value, json};

#[tokio::test]
async fn non_object_value_is_422() {
	let resp = http_common::send(
		http_common::state("http-val-a"),
		http_common::json_request("PUT", "/api/cfg/x", &json!([1, 2])),
	)
	.await;
	assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);
	let body: Value = serde_json::from_slice(&http_common::body_bytes(resp).await).expect("json");
	assert!(body["error"].as_str().expect("error").contains("invalid cfg value"));
}

#[tokio::test]
async fn unknown_schema_version_is_422() {
	let value = json!({ "schema_version": 2 });
	let resp = http_common::send(
		http_common::state("http-val-b"),
		http_common::json_request("PUT", "/api/cfg/x", &value),
	)
	.await;
	assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn missing_schema_version_is_422() {
	let value = json!({ "no_version": true });
	let resp = http_common::send(
		http_common::state("http-val-c"),
		http_common::json_request("PUT", "/api/cfg/x", &value),
	)
	.await;
	assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn malformed_json_is_400() {
	let req = Request::builder()
		.method("PUT")
		.uri("/api/cfg/x")
		.header("content-type", "application/json")
		.body(Body::from("not json"))
		.expect("request");
	let resp = http_common::send(http_common::state("http-val-d"), req).await;
	assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}
