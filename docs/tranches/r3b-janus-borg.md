# Tranche R-3b: janus_borg – Borg-Subprozess-Skelett (Dual-Version)

Status: **Implementierung freigegeben** (Auftraggeber bestätigt die
Ausrichtungs-Fluktuation als normal im Innovationsprojekt und billigt
das Fortfahren). Verträge DATAMODEL 0.5.0 / ARCHITECTURE 0.6.0 (Dual-
Version borg_version "1"/"2") bleiben bis zur namentlichen Freigabe als
ENTWURF markiert; die R-3b-Tranchenfreigabe ist hiermit erteilt.

## Live-Verifikation (beide Serien, echte Repos)

- **V2**: `/tmp/borg2env/bin/borg` = `borg 2.0.0b25`; Smoke-Repo
  `/tmp/janus-borg-smoke/testrepo.borg` (authenticated-blake3, Passphrase
  via BORG_PASSCOMMAND aus /tmp/janus-secrets/). `list --json-lines`
  liefert bei non-UTF8 zusätzlich `path_b64` (base64, byte-genau);
  `path` verstümmelt; `bpath`-Format-Key existiert nicht
  (`Invalid format keys: bpath`).
- **V1 Server (Auftraggeber)**: `ssh://borg@gw.oldfire.de:22222/…
  /kayda` – `borg --version` → `borg 1.4.0`; Auth SSH-Key
  `id_ed25519`; `list --json` → 71 Archive; `list --json-lines`
  Archiv `kayda-c54d7f19` → Felder type/mode/user/group/uid/gid/path/
  healthy/source/linktarget/flags/mtime/size. Passphrase nur via
  `BORG_PASSCOMMAND="cat /tmp/janus-secrets/kayda.pass"` (0600, außerhalb
  des Repos; nie in Argumenten/Logs/Commits/Verträgen).
- **V1 Byte-Fidelity (lokaler Live-Nachweis 1.4.5)**: Testdatei mit
  `\xff\xfe` im Namen: `--json-lines`-`path` = `bad??name` (literal `?`),
  **kein** `path_b64`-Feld (Feldliste vollständig geprüft);
  `--format '{path}'` verstümmelt ebenso. **Serie 1 hat keinen
  byte-genauen textuellen Pfadkanal.** → `path_fidelity`-Kennzeichnung
  + Subbaum-Fallback (DATAMODEL §1.3/§3.1).

## Umfang (Code erst nach Vertragfreigabe)

Neues Modul `janus_borg` + Tests + Cargo/CHANGELOG; RepoSpec-Schema 2
mit Pflichtfeld `borg_version` ("1"/"2"); REST durchgeschleift.

### janus_borg-Module (je ≤ 2000 Bytes, Tabs, SPDX, /// auf pub-Items)

- `mod.rs`: Re-Exports + Live-Fakten (beide Pins, path_b64 nur V2,
  V1-Verstümmelung, bpath entfallen, Subcommand-Renames V2).
- `error.rs`: `Spawn`, `Io(#[from])`, `Version{found,expected}`, `Exit{code,stderr}`.
- `version.rs`: `BorgSeries`-Enum `V1_4`/`V2_0B`; Pins `1.4.` bzw.
  `2.0.0b`; `check_version(&str)` prüft erste Zeile `borg <ver>` gegen
  den Pin der Serie; beide Serien sind zulässig, fremde nicht.
- `cmd.rs`: `BorgCmd{bin,series,args}`: `repo` (`-r <location>`; einheitlich
  V1+V2), `archive` (positional; V2 zusätzlich `aid:`-Instanz erlaubt),
  `repo_listing` (**seriengetrennt**: V1 `list --json`, V2 `repo-list
  --json`), `json_lines`, `build()`.
- `child.rs`: Eskalationsleiter TASKFRAMEWORK §2 (SIGTERM → Grace-Drain →
  SIGKILL → Reap; Reaped-Guard gegen PID-Wiederverwendung; Rückkehr erst
  nach tatsächlichem Ende); `next_stdout_line` (zeilenweise, bounded);
  `finish` (Drain beider Pipes, nonzero → `Exit`); `probe_version(bin)`
  → Serie-Erkennung.
- `drain` privat: stdout verwerfen, nur letzte stderr-Zeile behalten.

### janus_repo (Schema-Bump)

- `RepoSpec.schema_version` 1 → 2; neues Pflichtfeld `borg_version`
  (`"1"`/`"2"`, Werte-Validierung); keine stillschweigende Default-Serie.
  REST PUT/GET unverändert durchgeschleift; bestehende Tests ergänzt.

### Tests (Stubs deterministisch; Live env-gegated)

- Stub-Matrix Version: ok `borg 1.4.5`, ok `borg 2.0.0b13`, fail
  `borg 1.2.3`, fail `borg 3.0.0b1`, fail Müll.
- Adressierung: `["-r", "ssh://borg@gw.oldfire.de:22222/…/kayda",
  "kayda-c54d7f19", "--json-lines"]` (V1) und `aid:`-Form (V2).
- Streaming (5 JSONL-Zeilen → `None`), Cancel graceful (`trap 'exit 0'`),
  stur (`trap ''` → SIGKILL nach Grace), nach-Exit (`exit 7`; `cancel`
  danach deterministisch).
- Live: `JANUS_BORG1_BIN` (Server-Repo, volle Archivliste) und
  `JANUS_BORG2_BIN` (/tmp/borg2env); ohne Env deterministisch übersprungen.

### Cargo (janusd/)

tokio features + `process`, `io-util`, `time`; neu `nix 0.29`
(feature `signal`) für kill ohne unsafe.

## Churn-Schätzung (Schätzung, kein Messwert)

~16 Dateien: 5 Modul- + 6 Test-Dateien neu, 5 bestehende berührt
(lib.rs, Cargo.toml, Cargo.lock, CHANGELOG, RepoSpec-Modell + dessen
Tests). Risiko mittel: Cancel-Ladder-Timeouts und PID-Wiederverwendung
(Reaped-Guard) bleiben die kritischen Stellen; neu: Serien-Verzweigung
bei Repo-Listing (`list` vs `repo-list`) als Tabellen-Dispatch.
Beta-Drift V2 durch Pin+Smoke-Tests gebunden.
