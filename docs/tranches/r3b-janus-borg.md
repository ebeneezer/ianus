# Tranche R-3b: janus_borg – Borg-2-Subprozess-Skelett

Status: zur Implementierung freigegeben („weitermachen“, „Deine Entscheidung“);
Contract-Korrektur DATAMODEL 0.4.0 / ARCHITECTURE 0.5.0 läuft separat als ENTWURF.

## Live-Verifikation Borg 2.0.0b25 (lokale Installation, venv /tmp/borg2env)

- `borg --version` → `borg 2.0.0b25`
- `borg list --json-lines -r REPO ARCHIV`: bei non-UTF8-Pfaden zusätzlich Feld
  `path_b64` (base64, byte-genau); `path` verstümmelt (`??`-Ersatz).
  Smoke-Repo: `/tmp/janus-borg-smoke/testrepo.borg` (authenticated-blake3;
  Passphrase nur via `BORG_PASSCOMMAND`, nie Klartext/Args/Logs).
- `bpath` als `--format`-Key existiert nicht mehr: `Invalid format keys: bpath`.
- Repo-Ebene: `borg repo-list --json -r REPO` (umbenannte Subcommands
  `repo-create`/`repo-list`/`repo-info`; `--json`, kein `--json-lines`).
- `-e none` entfallen; Wahl: `aes256-ocb`, `chacha20-poly1305`,
  `authenticated-sha256`, `authenticated-blake3` (jeweils Passphrase nötig).
- Pin-Verifikation gegen den Borg-Server des Auftraggebers: offen
  (Substantielle Frage: Verbindungsdetails/Secret-Referenz für Server-Test).

## Umfang

Neues Modul `janus_borg` + Tests + Cargo/CHANGELOG. Keine HTTP-Endpunkte, kein
main.rs-Wiring (außer `pub mod janus_borg;` in lib.rs), keine Secrets-Logik,
kein Indexer, keine Änderungen an bestehenden Modulen.

### Module (jede Datei ≤ 2000 Bytes, Tabs, SPDX, /// auf allen pub-Items)

- `mod.rs`: Modul-Doku; Re-Exports `BorgChild`, `BorgCmd`, `Error`,
  `EXPECTED_PREFIX`, `probe_version`; Live-Fakten (path_b64, bpath entfallen,
  repo-list --json, Pin 2.0.0b25).
- `error.rs`: `Spawn(String)`, `Io(#[from] std::io::Error)`,
  `Version { found, expected }`, `Exit { code, stderr }`.
- `version.rs`: `EXPECTED_PREFIX = "2.0.0b"`; `check_version(&str) ->
  Result<String, Error>` (erste Zeile, Präfix `borg `, Serie prüfen).
- `cmd.rs`: `BorgCmd { bin, args }`: `new`, `repo` (`-r <location>`),
  `archive` (positional, kein `repo::archive`), `arg`, `json_lines`,
  `build()` → tokio::process::Command (stdin null, stdout/stderr piped).
- `child.rs`: `DEFAULT_GRACE = 5s`; `BorgChild` mit `spawn`, `set_grace`,
  `next_stdout_line` (zeilenweise, bounded), `last_stderr` (nur letzte Zeile),
  `finish` (drain beider Pipes bis EOF, dann `wait()` = Reap; nonzero →
  `Exit { code, stderr }`), `cancel` = Eskalationsleiter TASKFRAMEWORK §2:
  reaped? → gecachter Status (kein Signal an recycelten PID!) → `try_wait` →
  SIGTERM (nix::sys::signal::kill, KEIN unsafe im janusd-Code) → Grace-Drain
  mit `tokio::time::timeout(grace)` → sonst SIGKILL → drain → Reap.
  Rückkehr erst nach tatsächlichem Ende; Drop ohne cancel beendet das Kind
  nicht (kill_on_drop false, dokumentiert). `probe_version(bin)`:
  `--version` → sammeln (winzig) → `finish` → `check_version`.
- `drain(&mut self)` privat: stdout+stderr bis EOF, stdout verwerfen, nur
  letzte stderr-Zeile behalten (bounded).

### Tests (Stub-Skripte, deterministisch; je ≤ 2000 Bytes)

- `tests/borg_common/mod.rs`: `temp_dir(name)`, `stub_bor(dir, name, body)`
  (chmod 0755), `sleep_stub_body()`.
- `tests/borg_version.rs`: Unit-Checks (ok/mismatch/garbage) + Stub-Probe
  ok (`borg 2.0.0b13`) + mismatch (`borg 1.2.3`, Fehlermeldung enthält beide).
- `tests/borg_cmd.rs`: Echo-Args-Stub verifiziert
  `["-r", "ssh://u@h/p.borg", "aid:abc", "--json-lines"]` (Borg-2-Stil,
  echter Prozess).
- `tests/borg_stream.rs`: Stub 5 JSONL-Zeilen; 5× lesen, dann `None`;
  `finish` success.
- `tests/borg_cancel.rs` (ggf. splitten): graceful (`trap 'exit 0' TERM`;
  grace 2s → success, < 2s), stur (`trap '' TERM`; grace 300ms →
  `status.signal() == Some(9)`, ≥ 300ms), nach-Exit (`exit 7`; `finish`
  → `Exit { 7 }`, danach `cancel` deterministisch, kein Panic – Reaped-Guard).
- `tests/borg2_live.rs`: env-gegated (`JANUS_BORG2_BIN`); ohne Var
  deterministisch übersprungen; Aufruf z. B.
  `JANUS_BORG2_BIN=/tmp/borg2env/bin/borg cargo test --test borg2_live`.

### Cargo.toml (janusd/)

- tokio features: + `process`, `io-util`, `time`
- neu: `nix = { version = "0.29", features = ["signal"] }` (sichere
  kill-Kapselung statt libc/unsafe; bei Resolverproblemen 0.28, gleiches API)
- kein neues dev-dependency (eigene temp-Helfer)

### CHANGELOG ([Unreleased]/Added)

- janus_borg: Borg-2 subprocess skeleton – command builder (-r addressing,
  positional archives), bounded line streaming, SIGTERM→SIGKILL cancel ladder
  with grace, version probe mit 2.0.0b-Serien-Pin (2.0.0b25 lokal verifiziert)
- Tests via Stubs (Pin, Adressierung, Streaming, Cancel graceful/stur/nach-
  Exit); optionaler Live-Test via JANUS_BORG2_BIN

## Verifikationsreihenfolge

`export PATH="$HOME/.cargo/bin:$PATH"; cd janusd`
`cargo fmt && cargo fmt --check && cargo clippy --all-targets -- -D warnings
&& cargo test`; dann Dateigrößen prüfen (alle ≤ 2000 Bytes); bestehende
27 Tests müssen grün bleiben. Danach Live-Test gegen /tmp/borg2env/bin/borg.

## Churn-Schätzung und Risiko (Stand vor Implementierung)

- Churn: ~10 neue Dateien (5 Modul-, 5–6 Testdateien) + 3 bestehende
  (lib.rs, Cargo.toml, Cargo.lock, CHANGELOG) – Schätzung, kein Messwert.
- Risiko mittel: Cancel-Ladder (Timeout vs. Drain-Interaktion) und
  PID-Wiederverwendung (Reaped-Guard) sind die kritischen Stellen; beide
  durch Tests abgedeckt. Beta-Drift bleibt strukturell (Pin + Smoke-Tests).
