// SPDX-License-Identifier: GPL-3.0-or-later

//! REST skeleton over the cfg and repo stores (ARCHITECTURE §11; auth/RBAC later).

mod error;
mod routes;
mod routes_repo;

use axum::Router;
use serde::Deserialize;

use crate::janus_cfg::CfgStore;
use crate::janus_repo::RepoStore;

pub use error::ApiError;

/// Shared HTTP state.
#[derive(Clone)]
pub struct AppState {
	/// The cfg store backing the REST endpoints.
	pub cfg: CfgStore,
	/// The repo store backing the REST endpoints.
	pub repo: RepoStore,
}

/// Query parameters of the list endpoints (cfg and repos).
#[derive(Deserialize)]
pub struct ListQuery {
	/// Exclusive cursor: list entries after this name.
	pub after: Option<String>,
	/// Maximum number of entries (default 100, max 1000).
	pub limit: Option<u32>,
}

/// Builds the axum router with all REST routes.
pub fn router(state: AppState) -> Router {
	Router::new()
		.route("/api/health", axum::routing::get(routes::health))
		.route("/api/cfg", axum::routing::get(routes::list_cfg))
		.route(
			"/api/cfg/{key}",
			axum::routing::get(routes::get_cfg).put(routes::put_cfg),
		)
		.route("/api/repos", axum::routing::get(routes_repo::list_repos))
		.route(
			"/api/repos/{name}",
			axum::routing::get(routes_repo::get_repo)
				.put(routes_repo::put_repo)
				.delete(routes_repo::delete_repo),
		)
		.with_state(state)
}
