# Janus – Code-Style-Vertrag

Version: 0.2.0
Status: FREIGEGEBEN

## 1. C-Backend

### 1.1 Sprache & Standard

- C99 (`-std=c99`), POSIX.1-2008; keine GNU-Erweiterungen im Kerncode.
- Compiler: GCC oder Clang mit C99-Unterstützung.
- Warnungen: `-Wall -Wextra -Wpedantic -Werror` in CI.

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
- C-JSON-Parsing für große Borg-Ausgaben streamend; keine ganze große
  Borg-Antwort als DOM im Speicher halten.
- Cache begrenzt, mit explizitem Eigentum und Ref-Count-Leases.
- Debug-Logs speichern niemals Payloads oder Secrets.

### 1.3 Namenskonventionen

- Bezeichner beginnen niemals mit einem Unterstrich; reservierte C-Bezeichner bleiben unangetastet.
- Funktionen: `janus_<modul>_<verb>()` → `janus_db_open()`, `janus_repo_index()`.
- Typen: `janus_<name>_t` → `janus_repo_t`, `janus_job_state_t`.
- Enums: `JANUS_<MODUL>_<WERT>` → `JANUS_JOB_RUNNING`.
- Makros: `JANUS_<KONTEXT>_<NAME>` → `JANUS_DB_MAX_RETRIES`.
- Lokale Variablen: snake_case, keine Präfixe.
- Struct-Felder: snake_case.

### 1.4 Formatierung

- Einrückung ausschließlich mit Tabs; Tabstopps sind 3 Spalten breit.
- Klammerstil: K&R (öffnende Klammer auf derselben Zeile).
- Handgeschriebene Quellcodedateien dürfen höchstens 1000 Zeichen enthalten,
  einschließlich Leerraum und Kommentaren. Bei drohender Überschreitung wird der
  Code automatisch entlang semantisch zusammengehöriger Verantwortlichkeiten auf
  mehrere Dateien mit passenden Namen aufgeteilt oder refaktoriert. Die Aufteilung
  darf keine unnötigen Abstraktionen oder künstliche Fragmentierung erzeugen.
  Generierte und vendorte Dateien sind ausgenommen.
- clang-format-Konfiguration wird im Repo hinterlegt.

### 1.5 Speicherverwaltung

- Jede Allokation hat genau einen dokumentierten Eigentümer.
- `janus_<modul>_create()` erzeugt, `janus_<modul>_destroy()` gibt frei – symmetrisch.
- Rückgabewert bei Fehlern: `NULL` oder negativer int, kein `errno`-Overloading.
- Keine globalen Variablen außer read-only Konfiguration.

### 1.6 Fehlerbehandlung

- Funktionen geben `int` (0 = Erfolg, < 0 = Fehler) oder `NULL`-Pointer zurück.
- Fehlercodes als `JANUS_ERR_*`-Enums.
- Logging über `janus_log(level, fmt, ...)` mit sd-journal-Anbindung.

### 1.7 Dokumentation

- Jede öffentliche Funktion hat einen Doxygen-Kommentar über der Deklaration.
- Jede Datei hat einen SPDX-License-Identifier-Header.

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

- CMake >= 3.20 als Build-System.
- `compile_commands.json` wird generiert (für clangd/IDE-Integration).
- CI: GitHub Actions mit Build + Tests + clang-format-Check + clang-tidy.

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
