# Ianus – Code-Style-Vertrag

Version: 0.1.0
Status: ENTWURF – freigabepflichtig

## 1. C-Backend

### 1.1 Sprache & Standard
- C17 (`-std=c17`), POSIX.1-2008.
- Compiler: GCC >= 12 oder Clang >= 15.
- Warnungen: `-Wall -Wextra -Wpedantic -Werror` in CI.

### 1.2 Namenskonventionen
- Funktionen: `ianus_<modul>_<verb>()` → `ianus_db_open()`, `ianus_repo_index()`.
- Typen: `ianus_<name>_t` → `ianus_repo_t`, `ianus_job_state_t`.
- Enums: `IANUS_<MODUL>_<WERT>` → `IANUS_JOB_RUNNING`.
- Makros: `IANUS_<KONTEXT>_<NAME>` → `IANUS_DB_MAX_RETRIES`.
- Lokale Variablen: snake_case, keine Präfixe.
- Struct-Felder: snake_case.

### 1.3 Formatierung
- Einrückung: 4 Spaces, keine Tabs.
- Zeilenlänge: max. 100 Zeichen (weiche Grenze), 120 (harte Grenze).
- Klammerstil: K&R (öffnende Klammer auf derselben Zeile).
- clang-format-Konfiguration wird im Repo hinterlegt.

### 1.4 Speicherverwaltung
- Jede Allokation hat genau einen dokumentierten Eigentümer.
- `_create()` erzeugt, `_destroy()` gibt frei – symmetrisch.
- Rückgabewert bei Fehlern: `NULL` oder negativer int, kein `errno`-Overloading.
- Keine globalen Variablen außer read-only Konfiguration.

### 1.5 Fehlerbehandlung
- Funktionen geben `int` (0 = Erfolg, < 0 = Fehler) oder `NULL`-Pointer zurück.
- Fehlercodes als `IANUS_ERR_*`-Enums.
- Logging über `ianus_log(level, fmt, ...)` mit sd-journal-Anbindung.

### 1.6 Dokumentation
- Jede öffentliche Funktion hat einen Doxygen-Kommentar über der Deklaration.
- Jede Datei hat einen SPDX-License-Identifier-Header.

## 2. Frontend

- Festlegung in separatem FRONTEND.md (Framework-Wahl noch offen).
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
