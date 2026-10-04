// SPDX-License-Identifier: GPL-3.0-or-later

//! Command-line argument parsing for the janusd binary.

use std::env;

/// Default HTTP bind address.
const DEFAULT_BIND: &str = "127.0.0.1:8420";

/// Sentinel returned by [`parse`] when `--help` was requested.
pub const HELP_REQUESTED: &str = "--help";

/// Parsed command-line arguments.
pub struct Args {
	/// Address the HTTP server binds to.
	pub bind: String,
	/// Path of the config SQLite database.
	pub config_db: String,
}

/// Parses `env::args`; `--help` yields [`HELP_REQUESTED`].
pub fn parse() -> Result<Args, String> {
	let mut bind = DEFAULT_BIND.to_string();
	let mut config_db = None;
	let mut args = env::args().skip(1);
	while let Some(arg) = args.next() {
		match arg.as_str() {
			"--help" => return Err(HELP_REQUESTED.to_string()),
			"--bind" => bind = args.next().ok_or("--bind requires a value")?,
			"--config-db" => config_db = Some(args.next().ok_or("--config-db requires a value")?),
			other => return Err(format!("unknown argument: {other}")),
		}
	}
	let config_db = match config_db {
		Some(path) => path,
		None => default_config_db()?,
	};
	Ok(Args { bind, config_db })
}

/// Returns the usage text.
pub fn help() -> String {
	format!(
		"janusd {}\n\nUsage: janusd [OPTIONS]\n\nOptions:\n  --bind ADDR       Bind address (default: {DEFAULT_BIND})\n  --config-db PATH  Config database path\n  --help            Show this help\n",
		env!("CARGO_PKG_VERSION")
	)
}

/// Resolves the default config DB path from `XDG_DATA_HOME` or `HOME`.
fn default_config_db() -> Result<String, String> {
	if let Ok(dir) = env::var("XDG_DATA_HOME")
		&& !dir.is_empty()
	{
		return Ok(format!("{dir}/janusd/config.sqlite3"));
	}
	if let Ok(home) = env::var("HOME")
		&& !home.is_empty()
	{
		return Ok(format!("{home}/.local/share/janusd/config.sqlite3"));
	}
	Err("neither XDG_DATA_HOME nor HOME is set; pass --config-db".to_string())
}
