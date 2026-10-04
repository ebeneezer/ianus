# Janus – Code-Style-Vertrag

Version: 0.4.0
Status: FREIGEGEBEN

## 1. Backend (Rust)

### 1.1 Sprache & Standard

- Rust Edition 2024; stabile Toolchain (rustup); kein `unsafe` außer mit
  dokumentierter Begründung und Review-Vermerk (`// SAFETY:`-Kommentar reicht
  allein nicht; jede `unsafe`-Stelle ist im Review gesondert freizugeben).
- Warnungen als Fehler: `RUSTFLAGS="-D warnings"` in CI; `clippy` mit
  `-D warnings`; `rustfmt` mit Tab-Einrückung (Konfiguration in `rustfmt.toml`).
- Lints: `#![deny(missing_docs)]` für die öffentliche API des Daemon-Crates.

### 1.2 Dispatch und Effizienz

- Für Auswahl nach Operations-/Task-Typen tabellengesteuertes Dispatch bevorzugen;
  lange `if`-/`else if`-Ketten vermeiden.
- Kontrollfluss bleibt explizit: Tabellen nur einsetzen, wenn sie ihn vereinfachen,
  nicht für triviale oder semantisch unterschiedliche Fälle.
- Effizienz, Code-Wiederverwendung und geringer RSS-Verbrauch sind Entwurfsziele.
- Speicherbedarf begrenzen: große Borg-Ausgaben streamen und in begrenzten Batches
  verarbeiten; unnötige Kopien, dauerhaft gehaltene Daten und große Zwischenobjekte
  vermeiden.
- Keine Abstraktion ohne konkreten Bedarf. Gemeinsame Logik wiederverwenden,
  ohne zusätzliche Indirektion oder Laufzeitkosten ohne messbaren Nutzen einzuführen.
- Performance- und Speicheroptimierungen anhand reproduzierbarer Messungen
  validieren; keine Komplexität für hypothetische Engpässe hinzufügen.
- Store-Zugriff zentral über die **eine** kanonische Store API; keine
  Subsystem-APIs oder Bypässe.
- JSON-Parsing für große Borg-Ausgaben streamend (NDJSON zeilenweise);
  keine ganze große Borg-Antwort als DOM im Speicher halten.
- Cache begrenzt, mit explizitem Eigentum und Ref-Count-Leases.
- Debug-Logs speichern niemals Payloads oder Secrets.

### 1.3 Namenskonventionen

- Bezeichner beginnen niemals mit einem Unterstrich; keine Bezeichner, die
  mit `std_`, `core_`, `rust_` beginnen oder gegen Rust-Keywords verstoßen.
- Funktionen/Module: `janus_<modul>::janus_<verb>()` → `janus_db::open()`,
  `janus_repo::index()` (Crate-intern kurze `snake_case`-Namen erlaubt, wo
  der Modulpfad bereits den Kontext trägt).
- Typen: `PascalCase` → `JanusRepo`, `JobState`, `TaskInput`.
- Enums: `PascalCase` mit `PascalCase`-Varianten → `JobState::Running`.
- Konstanten: `SCREAMING_SNAKE_CASE` → `DB_MAX_RETRIES`.
- Lokale Variablen und Struct-Felder: snake_case, keine Präfixe.
- Modulnamen: snake_case, `janus_<thema>` → `janus_kv`, `janus_task`.

### 1.4 Formatierung

- Einrückung ausschließlich mit Tabs; Tabstopps sind 3 Spalten breit.
- Klammerstil: K&R (öffnende Klammer auf derselben Zeile).
- Handgeschriebene Quellcodedateien dürfen höchstens 2000 Zeichen enthalten
  (Anordnung des Auftraggebers: 2000 statt 1000, um Fragmentierung zu
  vermeiden), einschließlich Leerraum und Kommentaren. Bei drohender Überschreitung wird der
  Code automatisch entlang semantisch zusammengehöriger Verantwortlichkeiten auf
  mehrere Dateien mit passenden Namen aufgeteilt oder refaktoriert. Die Aufteilung
  darf keine unnötigen Abstraktionen oder künstliche Fragmentierung erzeugen.
  Generierte und vendorte Dateien sind ausgenommen.
- rustfmt-Konfiguration (`rustfmt.toml`, hard_tabs, tabsize 3) wird im Repo hinterlegt.

### 1.5 Speicherverwaltung

- Eigentum ist im Typsystem ausgedrückt (`Box`, `Arc`, `Rc`); jede Ressource
  hat genau einen dokumentierten Eigentümer oder einen expliziten
  Ref-Count-Pfad (Arc für Host-Cache-Leases).
- RAII: Acquisition/Release über Drop; keine manuellen free-Pfade.
- Keine globalen mutable States außer read-only Konfiguration (`OnceLock`
  für Unveränderliches).

### 1.6 Fehlerbehandlung

- Fehler als `thiserror`-Enums pro Modul (`janus_kv::Error`), keine Stringly-
  typed Errors; `anyhow` nur in `main`/Tests.
- `Result<T, E>` durchgängig; kein `.unwrap()`/`.expect()` außer in Tests
  oder nach bewiesener Invariante mit Begründung.
- Logging über `tracing` (Facilities `tracing-journald` für sd-journal).

### 1.7 Dokumentation

- Jede öffentliche API (pub) hat einen Rustdoc-Kommentar (`///`), Beispiele
  für nicht-triviale Funktionen (`/// # Examples`).
- Jede Quelldatei trägt einen SPDX-License-Identifier-Header.

## 2. Frontend

- Svelte 5 + Vite + TypeScript (verbindlich, siehe ARCHITECTURE §7); ESLint
  und Prettier-Konfiguration werden mit der Frontend-Tranche eingebracht.
- **JS-Quarantäne**: handgeschriebene `.js`-Dateien sind im Frontend-Quellbaum
  verboten (Linter/CI-Regel); nur `.svelte` und `.ts`, TypeScript `strict`,
  `any` verboten. JavaScript ausschließlich als Build-Artifact.
- ESLint + Prettier als Formatierer.
- Komponenten-Namen: PascalCase.
- Keine `any`-Types in TypeScript.

## 3. Build & CI

- cargo als Build-System; `Cargo.lock` wird committet.
- CI: GitHub Actions mit `cargo build`, `cargo clippy -- -D warnings`,
  `cargo fmt --check`, `cargo test`, `cargo audit` (Dependency-Schwachstellen).

## 4. Versionierung

- Semantic Versioning (MAJOR.MINOR.PATCH).
- Alle Vertragsdokumente tragen eine eigene Version.
- CHANGELOG.md im Repo-Root.

## 5. Commit-Konventionen

- Conventional Commits: `feat:`, `fix:`, `docs:`, `refactor:`, `test:`, `chore:`.
- Englische Commit-Messages (Imperativ: "Add ...", "Fix ...", nicht "Added").
- Jeder Commit kompiliert fehlerfrei.

---

**Dieses Dokument ist vor der ersten Implementierung vom Auftraggeber freizugeben.
Änderungen erfordern eine neue Version und erneute Freigabe.**
