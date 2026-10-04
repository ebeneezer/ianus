// SPDX-License-Identifier: GPL-3.0-or-later

//! HTTP round-trip tests for the repo endpoints.

mod common;
mod http_common;

use axum::http::StatusCode;
use serde_json::{Value, json};

#[tokio::test]
async fn put_get_delete_lifecycle() {
	let state = http_common::state("http-repo-rt");
	let spec = json!({"schema_version": 1, "name": "backup-server", "location": "ssh://user@host/path/repo.borg", "secret_ref": "backup-key", "label": "Backup server", "notes": "nightly"});
	let resp = http_common::send(
		state.clone(),
		http_common::json_request("PUT", "/api/repos/backup-server", &spec),
	)
	.await;
	assert_eq!(resp.status(), StatusCode::NO_CONTENT);

	let resp = http_common::send(
		state.clone(),
		http_common::empty_request("GET", "/api/repos/backup-server"),
	)
	.await;
	assert_eq!(resp.status(), StatusCode::OK);
	let body: Value = serde_json::from_slice(&http_common::body_bytes(resp).await).expect("json");
	assert_eq!(body, spec);

	let resp = http_common::send(
		state.clone(),
		http_common::empty_request("DELETE", "/api/repos/backup-server"),
	)
	.await;
	assert_eq!(resp.status(), StatusCode::NO_CONTENT);

	let resp = http_common::send(
		state.clone(),
		http_common::empty_request("GET", "/api/repos/backup-server"),
	)
	.await;
	assert_eq!(resp.status(), StatusCode::NOT_FOUND);

	let resp = http_common::send(state, http_common::empty_request("DELETE", "/api/repos/backup-server")).await;
	assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn get_missing_returns_404() {
	let resp = http_common::send(
		http_common::state("http-repo-missing"),
		http_common::empty_request("GET", "/api/repos/missing"),
	)
	.await;
	assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}
