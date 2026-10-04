// SPDX-License-Identifier: GPL-3.0-or-later

//! Shared helpers for HTTP integration tests.

use std::sync::Arc;

use axum::body::Body;
use axum::http::Request;
use axum::response::Response;
use http_body_util::BodyExt;
use janusd::janus_cfg::CfgStore;
use janusd::janus_http::{AppState, router};
use janusd::janus_repo::RepoStore;
use serde_json::Value;
use tower::ServiceExt;

/// Builds an app state over a fresh store for `name`.
pub fn state(name: &str) -> AppState {
	let store = Arc::new(crate::common::store(name));
	AppState {
		cfg: CfgStore::new(store.clone()),
		repo: RepoStore::new(store),
	}
}

/// Sends `req` through the router and returns the response.
pub async fn send(state: AppState, req: Request<Body>) -> Response {
	router(state).oneshot(req).await.expect("request")
}

/// Builds an empty request with the given method and URI.
#[allow(dead_code)]
pub fn empty_request(method: &str, uri: &str) -> Request<Body> {
	Request::builder()
		.method(method)
		.uri(uri)
		.body(Body::empty())
		.expect("request")
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
