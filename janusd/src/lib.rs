// SPDX-License-Identifier: GPL-3.0-or-later

//! janusd: the Janus backup daemon (tranche R-1 skeleton).
//!
//! Provides the canonical KV store API and its SQLite reference
//! adapter; the HTTP server arrives with tranche R-2.

#![deny(missing_docs)]

pub mod janus_kv;
