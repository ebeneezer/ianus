// SPDX-License-Identifier: GPL-3.0-or-later

//! Shared helpers for HTTP integration tests.

use std::sync::Arc;

use axum::body::Body;
use axum::http::Request;
use axum::response::Response;
use http_body_util::BodyExt;
use janusd::janus_cfg::CfgStore;
use janusd::janus_http::{AppState, router};
use serde_json::Value;
use tower::ServiceExt;

/// Builds an app state over a fresh store for `name`.
pub fn state(name: &str) -> AppState {
	AppState {
		cfg: CfgStore::new(Arc::new(crate::common::store(name))),
	}
}

/// Sends `req` through the router and returns the response.
pub async fn send(state: AppState, req: Request<Body>) -> Response {
	router(state).oneshot(req).await.expect("request")
}

/// Reads the response body as bytes.
pub async fn body_bytes(resp: Response) -> Vec<u8> {
	resp.into_body().collect().await.expect("body").to_bytes().to_vec()
}

/// Builds a JSON request with the given method, URI, and body.
pub fn json_request(method: &str, uri: &str, body: &Value) -> Request<Body> {
	Request::builder()
		.method(method)
		.uri(uri)
		.header("content-type", "application/json")
		.body(Body::from(serde_json::to_vec(body).expect("json")))
		.expect("request")
}
