# Changelog

All notable changes to this project are documented in this file.
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- janusd crate skeleton, janus_kv canonical KV store API (get/put/delete/exists/paged prefix-scan with cursor, atomic write_batch), SQLite reference adapter (WAL, busy-timeout), integration tests (tranche R-1).
- axum HTTP/REST skeleton (health, cfg get/put/list) (tranche R-2)
- janus_cfg: typed Janus self-configuration (cfg/ keyspace) over the canonical KV store API with schema_version validation
- HTTP integration tests via tower oneshot
- janus_repo: repo objects (repo/ keyspace) with location scheme validation (file/ssh/sftp/borg), secret references, strict schema; shared name validation (janus_ident)
- REST: GET/PUT/DELETE /api/repos and /api/repos/{name}
- HTTP integration tests for repo CRUD
