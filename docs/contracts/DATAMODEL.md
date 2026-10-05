# Janus – Datenmodell-Vertrag

Version: 0.5.0
Status: ENTWURF – Dual-Version-Support (Borg 1.4.x / 2.0.0b) nach Auftraggeber-Anordnung und Live-Verifikation; erneute Freigabe erforderlich

## 1. Kernentitäten

Die nachfolgend als Tabellen beschriebenen Entitäten sind **logische
Entitäten/Records** – keine Anforderung an eine duplizierte relationale
Tabellenpersistenz. Physische Datenhaltung ist ausschließlich das KV-Backend
(siehe §2); die Tabellenform beschreibt nur die logische Struktur.

### 1.1 Repository (`repos`)

| Feld                  | Typ              | Beschreibung                                               |
|-----------------------|------------------|------------------------------------------------------------|
| id                    | UUID/INTEGER     | Primärschlüssel                                            |
| name                  | TEXT             | Anzeigename                                                |
| transport             | ENUM             | `local`, `ssh`, `sftp`, `borg_serve`                       |
| host                  | TEXT NULL        | Hostname (bei Remote)                                      |
| port                  | INTEGER NULL     | Port (bei Remote)                                          |
| user                  | TEXT NULL        | Benutzer (bei Remote)                                      |
| path                  | TEXT            | Pfad zum Repository                                        |
| borg_version          | ENUM            | Ziel-Borg-Serie: "1" (1.4.x, Server des Auftraggebers) oder "2" (2.0.0b); pro Repo/Host-Konfiguration umschaltbar (Auftraggeberanordnung – V2-only-Zug widerrufen) |
| secret_ref            | TEXT NULL        | Referenz auf Passphrase/Schlüssel                          |
| last_indexed_at       | TIMESTAMP        | Letzter erfolgreicher Index-Lauf                           |
| current_generation_id | FK → generations | Aktuell veröffentlichte Generation (NULL vor erstem Index) |
| created_at            | TIMESTAMP        | Erstellungszeitpunkt                                       |

### 1.2 Archive (`archives`)

| Feld         | Typ          | Beschreibung                               |
|--------------|--------------|--------------------------------------------|
| id           | UUID/INTEGER | Primärschlüssel                            |
| repo_id      | FK → repos   | Zugehöriges Repository                     |
| aid          | TEXT         | Borg-2-`aid:` (eindeutige Instanz)                     |
| name         | TEXT         | Serienname (Borg 2: nicht eindeutig)      |
| hostname     | TEXT         | Quell-Hostname                             |
| created_at   | TIMESTAMP    | Erstellungszeitpunkt im Borg-Archiv        |
| nfiles       | BIGINT       | Anzahl Dateien                             |
| size_original| BIGINT       | Originalgröße in Bytes                     |
| size_dedup   | BIGINT       | Deduplizierte Größe                        |

### 1.3 Pfad-Index (`paths`)

Dies ist die zentrale Tabelle für den "Merged Filesystem Tree":

| Feld         | Typ          | Beschreibung                               |
|--------------|--------------|--------------------------------------------|
| id           | BIGINT       | Primärschlüssel                            |
| archive_id   | FK → archives| In welchem Archiv dieser Eintrag existiert |
| path         | TEXT         | Vollständiger Pfad (UTF-8)                 |
| parent_path  | TEXT         | Elternverzeichnis (für Baum-Queries)       |
| type         | ENUM         | `file`, `dir`, `symlink`, `special`        |
| size         | BIGINT       | Größe in Bytes                             |
| mtime        | TIMESTAMP    | Änderungszeitpunkt                         |
| mode         | INTEGER      | POSIX-Berechtigungen                       |
| uid          | INTEGER      | Eigentümer-UID                             |
| gid          | INTEGER      | Gruppen-GID                                |
| hash         | TEXT NULL     | Content-Hash (wenn von Borg geliefert)     |
| path_fidelity | ENUM         | `exact` (V2: `path_b64`) oder `mangled` (V1: `?`-Ersatz); V1-Items nie stillschweigend als exakt behandelt |

**Indizes**:
- `(repo_id, path, Archiv-Rangfolge)` – Rang nach Archivfolge, nicht mtime – für "aktuellste Version pro Pfad"
- `(repo_id, parent_path)` – für Verzeichnis-Listing
- `(archive_id)` – für Archiv-bezogene Abfragen

**Logische Abfrage: Aktueller Wald** (`current_tree`)

Der `current_tree` ist eine **logische Abfrage**, kein operatives SQL-View:
Sobald das KV-Backend kanonisch ist, kann kein SQL-View als zweites Modell
bestehen bleiben. Semantik: Für jedes Repository wird über die aktuelle
veröffentlichte Generation (Generation-Mitgliedschaft) je Pfad der Pfadeintrag aus dem jüngsten Archiv der Generation, das den
Pfad enthält, zurückgegeben – Rangfolge der Archive nach Anlege-Reihenfolge;
Datei-mtime ist Metadatum, nie Rangkriterium. Die
Implementierung nutzt API-Prefix-Scans und das Index-Design der Store API –
kein SQL-View als zweites Datenmodell. Der Baum berücksichtigt ausschließlich
Archive der vollständig veröffentlichten aktuellen Generation.

### 1.4 Generation (`generations`)

Eine Generation ist pro Repository ein **unveränderlicher, konsistenter
Index-Snapshot** der zu einem Zeitpunkt bekannten Archive. Sie enthält nur
Metadaten und Referenzen auf bestehende Archive und Pfade – keine Kopien von
Borg-Daten oder Pfadmetadaten.

| Feld       | Typ          | Beschreibung                      |
|------------|--------------|-----------------------------------|
| id         | UUID/INTEGER | Primärschlüssel                   |
| repo_id    | FK → repos   | Zugehöriges Repository            |
| created_at | TIMESTAMP    | Erstellungszeitpunkt              |
| state      | ENUM         | `building`, `published`, `failed` |

Die **Generation-Archiv-Mitgliedschaft** wird als Referenzen auf Archive-IDs
abgebildet (`generation_archives (generation_id, archive_id)`); die Pfade
bleiben unverändert in `paths` und werden über ihre `archive_id` der Generation
zugeordnet.

`repos.current_generation_id` zeigt auf die aktuell veröffentlichte Generation.
Eine neue Generation wird zunächst nicht sichtbar aufgebaut (`building`) und
erst nach erfolgreichem Indexieren atomar als `published` veröffentlicht und
als `current_generation_id` gesetzt. Bei einem Fehler bleibt die bisherige
aktuelle Generation verfügbar. Historische Generationen bleiben für bereits
erstellte Restore-Jobs referenzierbar.

### 1.5 Benutzer (`users`)

| Feld          | Typ          | Beschreibung                             |
|---------------|--------------|------------------------------------------|
| id            | UUID/INTEGER | Primärschlüssel                          |
| username      | TEXT UNIQUE  | Login-Name                               |
| display_name  | TEXT         | Anzeigename                              |
| password_hash | TEXT         | bcrypt/argon2-Hash                       |
| enabled       | BOOLEAN      | Konto aktiv?                             |
| created_at    | TIMESTAMP    | Erstellungszeitpunkt                     |

### 1.6 Gruppen (`groups`)

| Feld | Typ          | Beschreibung       |
|------|--------------|--------------------|
| id   | UUID/INTEGER | Primärschlüssel    |
| name | TEXT UNIQUE  | Gruppenname        |

### 1.7 Rollen (`roles`)

| Feld        | Typ          | Beschreibung                            |
|-------------|--------------|-----------------------------------------|
| id          | UUID/INTEGER | Primärschlüssel                         |
| name        | TEXT UNIQUE  | Rollenname                              |
| permissions | TEXT         | Komma-separierte Liste oder JSON-Array  |
| scope_type  | ENUM         | `global`, `repo`, `host`                |
| scope_id    | TEXT NULL    | Bei repo/host: ID des Scope-Objekts     |

### 1.8 Zuordnungstabellen

- `user_groups (user_id, group_id)`
- `user_roles (user_id, role_id)`
- `group_roles (group_id, role_id)`

### 1.9 Jobs (`jobs`)

| Feld         | Typ          | Beschreibung                               |
|--------------|--------------|--------------------------------------------|
| id           | UUID/INTEGER | Primärschlüssel                            |
| type         | TEXT         | Task-Typ (`restore`, `check`, `prune`, …)  |
| state        | ENUM         | `draft`, `queued`, `running`, `done`, `failed`, `cancelled` |
| created_by   | FK → users   | Ersteller                                  |
| created_at   | TIMESTAMP    | Erstellungszeitpunkt                       |
| started_at   | TIMESTAMP    | Startzeit                                  |
| finished_at  | TIMESTAMP    | Endzeit                                    |
| params       | TEXT (JSON)  | Task-spezifische Parameter                 |
| result       | TEXT (JSON)  | Ergebnis/Fehlermeldung                     |
| progress_pct | INTEGER      | Fortschritt 0–100                          |

### 1.10 Job-Items (`job_items`)

| Feld          | Typ              | Beschreibung                                   |
|---------------|------------------|------------------------------------------------|
| id            | BIGINT           | Primärschlüssel                                |
| job_id        | FK → jobs        | Zugehöriger Job                                |
| generation_id | FK → generations | Gepinnte Generation                            |
| repo_id       | FK → repos       | Repository                                     |
| archive_id    | FK → archives    | Archiv der gepinnten Version                   |
| path_id       | FK → paths       | Referenzierter Pfadeintrag (konkrete Version!) |

Jedes Restore-Item hält **Generation + konkrete Pfadversion** fest
(`generation_id`, `repo_id`, `archive_id`, `path_id` – Zeilen-IDs), nicht
dynamisch die "aktuellste Version". Ein Job kann Items aus unterschiedlichen
Repositories und Generationen enthalten; das Restore-Ziel bleibt ein
Job-Parameter. Pfadobjekte werden nicht dupliziert – `path_id` referenziert die
bestehende Zeile in `paths`.

Dies bildet das "auf den Job gezogene Objekte" im UI ab.

## 2. KV-Schlüssel und JSON-Werte

### 2.1 JSON-Werte

- Jeder KV-Wert ist ein **JSON-Objekt** (ein Objekt pro Wert).
- Jedes Objekt enthält `schema_version`.
- 64-Bit-Größen und IDs werden gegenüber JS als **Dezimal-Strings** exponiert
  (kein IEEE-754-Rounding; IDs bleiben stabiler Text).
- Pfad-Bytes, die kein gültiges UTF-8 sind, werden **verlustfrei** repräsentiert,
  niemals stillschweigend replacement-decodiert.
- Ein JSON-Wert ist eine **begrenzte einzelne Entität/Record** – nie ein ganzer
  Host/Baum und nie eine ganze Borg-Ausgabe.
- Das KV-Backend ist der **einzige persistente Wahrheitsbestand**; keine
  duplizierten dauerhaften Kopien in SQL-Tabellen oder im KV-Cache.

### 2.2 Schlüssel

Schlüssel sind stabile, nicht-JSON-hierarchische Namespaces und Identifikatoren:

| Namespace | Schlüsselform |
|-----------|---------------|
| Repository | `repo/<repo-id>` |
| Archiv | `archive/<repo-id>/<archive-id>` |
| Generation | `generation/<repo-id>/<generation-id>` |
| Generation-Archiv-Mitgliedschaft | `generation-archive/<repo-id>/<generation-id>/<archive-id>` |
| Pfad | `path/<repo-id>/<archive-id>/<encoded-relative-path-segments>` |
| Job | `job/<job-id>` |
| Job-Item | `job-item/<job-id>/<item-id>` |
| Janus-Konfiguration | `cfg/<schlüssel>` |

- Pfadsegmente benötigen eine **verlustfreie Kodierung** für beliebige
  POSIX-Bytes und eine trenner-sichere Hierarchie; der benutzersichtbare
  Rohpfad ist als Schlüssel ungeeignet (mehrdeutig). Die präzise Kodierung
  ist **eine einzige festgelegte Kodierung**: empfohlen wird base64url ohne
  Padding, getrennt pro Pfadsegment, mit `/` als Trenner.
- Logische Generation-Mitgliedschaft und Pfadversions-Referenzen bleiben
  erhalten; keine duplizierten JSON-Pfadmetadaten nur für Cache/Index.

### 2.3 Store API

- **Eine einzige öffentliche Access API** (Rust-Trait `janus_kv::KvStore`,
  Tranche R-1) mit `get`, `put`, `delete`,
  `prefix-scan` (begrenzte Seiten/Cursor) sowie Batch-/Transaktionssemantik.
- Backend-Adapter sind intern; es werden **keine** mehreren Subsystem-APIs
  oder Bypässe erzeugt.
- Die aktuellen Schlüssel/Ordnung müssen Repo-, Generation-, Archiv- und
  Pfadzugriffe unterstützen.
- **Nicht überzeichnen**: Ein effizienter "aktuellster Pfad pro Pfad"-Zugriff
  entsteht nicht automatisch. Offene Anforderung: ein getesteter
  Query-/Index-Plan und ein Benchmark mit großem Datensatz vor der
  Implementierung.

### 2.4 Runtime Host Cache

- Der Runtime Host Cache ist **abgeleitet und volatil**; ein offener
  Host/Repo hält einen **Lease**.
- Records werden **on-demand in begrenzten Seiten** in einen indizierten
  In-Memory-Cache geladen; Verwendung für Filterung und Job-Zusammenstellung.
- `close` gibt den Lease frei und startet einen kurzen Grace-Timer; evictiert
  wird erst nach Ablauf der Frist und ohne UI-/Job-Lease.
- Job-Items persistieren gepinnte IDs/Generation im KV und müssen bei
  fehlendem Cache neu geladen werden.
- Cache-Daten sind **nie autoritativ**: Schreibzugriffe gehen zuerst durch
  die Store API, danach wird der Cache konsistent aktualisiert/invalidiert;
  kein stiller Dirty-Write-Back.
- Speicher sorgfältig begrenzen; kein unbegrenzter Whole-Host-Load.
- Keine separaten persistenten Cache-Entitäten einführen.

### 2.5 Janus-Eigenkonfiguration (`cfg/`)

- Die Janus-Eigenkonfiguration (Color-Theme, UI-Sprache, künftige
  Einstellungen) liegt in einem **eigenen lokalen SQLite-Speicher** hinter
  derselben Store API – bootstrap-fähig, auch wenn ein entferntes
  DB-Backend unerreichbar ist.
- Eigenkonfiguration und Indexbestand sind **disjunkte Datenmengen**; das
  ist keine doppelte Datenhaltung.
- UI-Übersetzungstexte und Theme-Definitionen sind **statische
  Frontend-Ressourcen**; im Store liegen nur die gewählten Werte (Sprache,
  Theme) als versionierte JSON-Objekte.

## 3. Indexierungsstrategie

1. **Vollindexierung**: Die Borg-Serie steht pro Repo über
   `repos.borg_version` fest ("1"/"2", Auftraggeberanordnung – der
   einzige produktive Borg-Server läuft 1.4.0). Beim Hinzufügen eines
   Repos werden alle Archive eingelesen – Repository via `-r`/
   `BORG_REPO`, Archiv positional.
   - **Borg 2 (2.0.0b25, lokal live verifiziert)**: **Byte-Pfadtreue
     über `path_b64` in `--json-lines`**: Bei non-UTF8-Pfaden liefert
     `borg list --json-lines` zusätzlich `path_b64` (base64,
     byte-genau), während `path` verstümmelt ist; der Indexer liest
     `path_b64`, falls vorhanden, sonst `path`. Der `bpath`-Format-Key
     existiert nicht mehr (`Invalid format keys: bpath`). Nachweis mit
     realen non-UTF8-Dateinamen: **Restore-Identitäten sind
     byte-stabil.**
   - **Borg 1 (Server 1.4.0 / Client 1.4.5, live verifiziert)**: **kein
     `path_b64`** (Feld existiert in 1.4.x nicht); `path` verstümmelt
     non-UTF8 zu `?` – `--format` ebenfalls (Live-Nachweis). Es gibt
     **keinen byte-genauen textuellen Kanal** in Serie 1. Konsequenz:
     Restore-Identitäten sind für **UTF-8-Pfade** stabil; non-UTF8-
     Einträge werden mit **Fidelity-Kennzeichnung** indexiert
     (`path_fidelity`-Feld), Restore des Einzelpfads ist unzuverlässig
     und wird als solcher ausgewiesen; dokumentierter Fallback ist der
     Subbaum-Restore über den letzten vollständig UTF-8-fähigen
     Vorfahren. Migrationsempfehlung: Server-Überführung auf Borg 2
     (`borg transfer`, Phase 3) – der Fidelity-Unterschied ist der
     wesentliche Grund, Serie 2 als Ziel zu führen.
2. **Generationen**: Jeder Index-Lauf erzeugt und füllt eine neue Generation
   (`state = building`). Erst nach erfolgreichem Indexieren wird sie atomar
   als `published` veröffentlicht und als `current_generation_id` gesetzt;
   bei einem Fehler bleibt die bisherige aktuelle Generation verfügbar.
3. **Inkrementelle Indexierung**: Bei erneutem Index-Lauf wird die
   vollständige Archivliste des Repos mit dem Index abgeglichen: neu ist,
   was im Repo existiert, aber nicht indexiert ist; entfernt ist, was
   indexiert war, aber nicht mehr im Repo existiert (erkannte Löschungen
   werden aus der neuen Generation ausgeschlossen). Kein ID-Vergleich
   (`id > last_indexed` – erkennt Löschungen nicht). Die neue Generation
   referenziert die weiterhin existierenden indexierten Archive, ohne
   deren Pfadmetadaten doppelt zu speichern.
4. **Hintergrund**: Der Indexer läuft als Thread im Daemon oder als
   separater Timer-getriggerter Prozess.
5. **Größenordnung**: Der Pfad-Index kann bei großen Repos Millionen Zeilen
   umfassen. Batch-Inserts innerhalb einer Transaktion sind Pflicht.

## 4. DB-Portabilität

- Alle Timestamps als UTC, ISO-8601 TEXT (SQLite) bzw. `TIMESTAMPTZ` (PostgreSQL).
- UUIDs als TEXT (SQLite) bzw. native UUID (PostgreSQL).
- ENUM-Felder als TEXT mit CHECK-Constraint (SQLite) bzw. native ENUM (PostgreSQL).
- Migrationen als nummerierte `.sql`-Dateien unter `db/migrations/`.

---

**Dieses Dokument ist vor der ersten Implementierung vom Auftraggeber freizugeben.
Änderungen erfordern eine neue Version und erneute Freigabe.**
