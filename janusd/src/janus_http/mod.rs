// SPDX-License-Identifier: GPL-3.0-or-later

//! REST skeleton over the cfg store (ARCHITECTURE §11; auth/RBAC later).

mod error;
mod routes;

use axum::Router;

use crate::janus_cfg::CfgStore;

pub use error::ApiError;

/// Shared HTTP state.
#[derive(Clone)]
pub struct AppState {
	/// The cfg store backing the REST endpoints.
	pub cfg: CfgStore,
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
		.with_state(state)
}
