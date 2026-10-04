// SPDX-License-Identifier: GPL-3.0-or-later

//! janusd binary entry point: HTTP/REST server over the KV store.

mod args;

use std::path::Path;
use std::process::ExitCode;
use std::sync::Arc;

use janusd::janus_cfg::CfgStore;
use janusd::janus_http::{AppState, router};
use janusd::janus_kv::SqliteStore;
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

/// Boxed error for the top-level [`run`] function.
type BoxError = Box<dyn std::error::Error>;

#[tokio::main]
async fn main() -> ExitCode {
	match run().await {
		Ok(()) => ExitCode::SUCCESS,
		Err(err) => {
			tracing::error!("startup failed: {err}");
			ExitCode::FAILURE
		}
	}
}

/// Starts the HTTP server; returns on shutdown or error.
async fn run() -> Result<(), BoxError> {
	tracing_subscriber::fmt()
		.with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
		.init();

	let args = match args::parse() {
		Ok(args) => args,
		Err(msg) if msg == args::HELP_REQUESTED => {
			print!("{}", args::help());
			return Ok(());
		}
		Err(msg) => return Err(msg.into()),
	};

	let config_path = Path::new(&args.config_db);
	if let Some(parent) = config_path.parent() {
		std::fs::create_dir_all(parent)?;
	}
	let store = Arc::new(SqliteStore::open(config_path)?);
	let state = AppState {
		cfg: CfgStore::new(store),
	};
	let listener = TcpListener::bind(&args.bind).await?;
	tracing::info!("janusd listening on {} (config db: {})", args.bind, args.config_db);
	axum::serve(listener, router(state)).await?;
	Ok(())
}
