// SPDX-License-Identifier: GPL-3.0-or-later

//! REST handlers for health and cfg endpoints.

use axum::Json;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use serde::Deserialize;

use crate::janus_cfg::CfgEntry;
use crate::janus_http::{ApiError, AppState};

/// Query parameters of the cfg list endpoint.
#[derive(Deserialize)]
pub struct ListQuery {
	/// Exclusive cursor: list entries after this name.
	pub after: Option<String>,
	/// Maximum number of entries (default 100, max 1000).
	pub limit: Option<u32>,
}

/// Returns `{"status":"ok"}`.
pub async fn health() -> Json<serde_json::Value> {
	Json(serde_json::json!({ "status": "ok" }))
}

/// Returns the value of `cfg/<key>`, or 404 if absent.
pub async fn get_cfg(
	State(state): State<AppState>,
	Path(name): Path<String>,
) -> Result<Json<serde_json::Value>, ApiError> {
	match state.cfg.get(&name).await? {
		Some(value) => Ok(Json(value)),
		None => Err(ApiError::not_found(format!("cfg key not found: {name}"))),
	}
}

/// Validates and stores a cfg value; returns 204 No Content.
pub async fn put_cfg(
	State(state): State<AppState>,
	Path(name): Path<String>,
	payload: Result<Json<serde_json::Value>, axum::extract::rejection::JsonRejection>,
) -> Result<StatusCode, ApiError> {
	let value = payload.map_err(|_| ApiError::bad_request("invalid JSON body"))?;
	state.cfg.put(&name, &value.0).await?;
	Ok(StatusCode::NO_CONTENT)
}

/// Lists cfg entries, optionally after a cursor with a limit.
pub async fn list_cfg(
	State(state): State<AppState>,
	Query(q): Query<ListQuery>,
) -> Result<Json<Vec<CfgEntry>>, ApiError> {
	let limit = q.limit.unwrap_or(100);
	if !(1..=1000).contains(&limit) {
		return Err(ApiError::bad_request("limit must be between 1 and 1000"));
	}
	let entries = state.cfg.list(q.after.as_deref(), limit).await?;
	Ok(Json(entries))
}
