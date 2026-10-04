// SPDX-License-Identifier: GPL-3.0-or-later

//! REST handlers for the repo endpoints.

use axum::Json;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use serde_json::Value as JsonValue;

use crate::janus_http::{ApiError, AppState, ListQuery};
use crate::janus_repo::{RepoEntry, RepoSpec};

/// Returns the spec of `repo/<name>`, or 404 if absent.
pub async fn get_repo(State(state): State<AppState>, Path(name): Path<String>) -> Result<Json<RepoSpec>, ApiError> {
	match state.repo.get(&name).await? {
		Some(spec) => Ok(Json(spec)),
		None => Err(ApiError::not_found(format!("repo not found: {name}"))),
	}
}

/// Validates and stores a repo spec; returns 204 No Content.
pub async fn put_repo(
	State(state): State<AppState>,
	Path(name): Path<String>,
	payload: Result<Json<JsonValue>, axum::extract::rejection::JsonRejection>,
) -> Result<StatusCode, ApiError> {
	let value = payload.map_err(|_| ApiError::bad_request("invalid JSON body"))?;
	let bytes = serde_json::to_vec(&value.0).map_err(|_| ApiError::internal("json serialization failed"))?;
	let spec = RepoSpec::from_json(&bytes)?;
	state.repo.put(&name, &spec).await?;
	Ok(StatusCode::NO_CONTENT)
}

/// Deletes `repo/<name>`; 204 on success, 404 if absent.
pub async fn delete_repo(State(state): State<AppState>, Path(name): Path<String>) -> Result<StatusCode, ApiError> {
	if state.repo.delete(&name).await? {
		Ok(StatusCode::NO_CONTENT)
	} else {
		Err(ApiError::not_found(format!("repo not found: {name}")))
	}
}

/// Lists repo entries, optionally after a cursor with a limit.
pub async fn list_repos(
	State(state): State<AppState>,
	Query(q): Query<ListQuery>,
) -> Result<Json<Vec<RepoEntry>>, ApiError> {
	let limit = q.limit.unwrap_or(100);
	if !(1..=1000).contains(&limit) {
		return Err(ApiError::bad_request("limit must be between 1 and 1000"));
	}
	let entries = state.repo.list(q.after.as_deref(), limit).await?;
	Ok(Json(entries))
}
