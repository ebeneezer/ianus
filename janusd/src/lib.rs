// SPDX-License-Identifier: GPL-3.0-or-later

//! janusd: the Janus backup daemon (tranche R-2 skeleton).
//!
//! Provides the canonical KV store API with its SQLite reference
//! adapter, typed Janus self-configuration (`janus_cfg`), and the
//! axum HTTP/REST skeleton (`janus_http`).

#![deny(missing_docs)]

pub mod janus_cfg;
pub mod janus_http;
pub mod janus_kv;
