// SPDX-License-Identifier: GPL-3.0-or-later

//! janus_borg: subprocess control for pinned Borg series (dual support).
//!
//! Two series are supported and pinned per repository:
//! [`BorgSeries::V1_4`](BorgSeries::V1_4) (client 1.4.x, maintainer server
//! 1.4.0) and [`BorgSeries::V2_0B`](BorgSeries::V2_0B) (2.0.0b series,
//! verified at 2.0.0b25).
//!
//! Live-verified behaviour (real repositories, tranche R-3b):
//! - V2: `list --json-lines` adds `path_b64` (base64, byte-exact) for
//!   non-UTF8 names; `path` is mangled; the `bpath` format key is gone.
//! - V1: no `path_b64` at all; `path` and `--format '{path}'` mangle
//!   non-UTF8 names to literal `?` (fidelity is marked downstream).
//! - V2 repo-level listing uses `repo-list --json`; V1 uses `list --json`.
//!
//! Cancellation follows TASKFRAMEWORK §2: SIGTERM, bounded grace drain,
//! SIGKILL, then reap; the call returns only after the child truly ended.

pub mod child;
pub mod cmd;
pub mod drain;
pub mod error;
pub mod ladder;
pub mod probe;
pub mod stream;
pub mod version;

pub use child::{BorgChild, DEFAULT_GRACE};
pub use cmd::BorgCmd;
pub use error::Error;
pub use probe::probe_version;
pub use version::{BorgSeries, series_from_version_string};
