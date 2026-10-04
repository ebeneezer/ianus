# Ianus – Datenmodell-Vertrag

Version: 0.1.0
Status: ENTWURF – freigabepflichtig

## 1. Kernentitäten

### 1.1 Repository (`repos`)

| Feld            | Typ          | Beschreibung                              |
|-----------------|--------------|-------------------------------------------|
| id              | UUID/INTEGER | Primärschlüssel                           |
| name            | TEXT         | Anzeigename                               |
| transport       | ENUM         | `local`, `ssh`, `sftp`, `borg_serve`      |
| host            | TEXT NULL    | Hostname (bei Remote)                     |
| port            | INTEGER NULL | Port (bei Remote)                         |
| user            | TEXT NULL    | Benutzer (bei Remote)                     |
| path            | TEXT         | Pfad zum Repository                       |
| secret_ref      | TEXT NULL    | Referenz auf Passphrase/Schlüssel         |
| last_indexed_at | TIMESTAMP    | Letzter erfolgreicher Index-Lauf          |
| created_at      | TIMESTAMP    | Erstellungszeitpunkt                      |

### 1.2 Archive (`archives`)

| Feld         | Typ          | Beschreibung                               |
|--------------|--------------|--------------------------------------------|
| id           | UUID/INTEGER | Primärschlüssel                            |
| repo_id      | FK → repos   | Zugehöriges Repository                     |
| borg_id      | TEXT         | Borg-interne Archiv-ID                     |
| name         | TEXT         | Archivname (z. B. Hostname-Timestamp)      |
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

**Indizes**:
- `(repo_id, path, mtime DESC)` – für "aktuellste Version pro Pfad"
- `(repo_id, parent_path)` – für Verzeichnis-Listing
- `(archive_id)` – für Archiv-bezogene Abfragen

**View: Aktueller Wald** (`current_tree`):
```sql
SELECT DISTINCT ON (r.id, p.path)
       r.id AS repo_id, r.name AS repo_name,
       a.hostname, p.*
FROM   paths p
JOIN   archives a ON a.id = p.archive_id
JOIN   repos r    ON r.id = a.repo_id
ORDER  BY r.id, p.path, p.mtime DESC;
```
(SQLite-Variante mit Window-Funktion `ROW_NUMBER()` + Subquery.)

### 1.4 Benutzer (`users`)

| Feld          | Typ          | Beschreibung                             |
|---------------|--------------|------------------------------------------|
| id            | UUID/INTEGER | Primärschlüssel                          |
| username      | TEXT UNIQUE  | Login-Name                               |
| display_name  | TEXT         | Anzeigename                              |
| password_hash | TEXT         | bcrypt/argon2-Hash                       |
| enabled       | BOOLEAN      | Konto aktiv?                             |
| created_at    | TIMESTAMP    | Erstellungszeitpunkt                     |

### 1.5 Gruppen (`groups`)

| Feld | Typ          | Beschreibung       |
|------|--------------|--------------------|
| id   | UUID/INTEGER | Primärschlüssel    |
| name | TEXT UNIQUE  | Gruppenname        |

### 1.6 Rollen (`roles`)

| Feld        | Typ          | Beschreibung                            |
|-------------|--------------|-----------------------------------------|
| id          | UUID/INTEGER | Primärschlüssel                         |
| name        | TEXT UNIQUE  | Rollenname                              |
| permissions | TEXT         | Komma-separierte Liste oder JSON-Array  |
| scope_type  | ENUM         | `global`, `repo`, `host`                |
| scope_id    | TEXT NULL    | Bei repo/host: ID des Scope-Objekts     |

### 1.7 Zuordnungstabellen

- `user_groups (user_id, group_id)`
- `user_roles (user_id, role_id)`
- `group_roles (group_id, role_id)`

### 1.8 Jobs (`jobs`)

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

### 1.9 Job-Items (`job_items`)

| Feld     | Typ          | Beschreibung                                 |
|----------|--------------|----------------------------------------------|
| id       | BIGINT       | Primärschlüssel                              |
| job_id   | FK → jobs    | Zugehöriger Job                              |
| path_id  | FK → paths   | Referenzierter Pfadeintrag (Version!)        |
| repo_id  | FK → repos   | Repository                                   |

Dies bildet das "auf den Job gezogene Objekte" im UI ab.

## 2. Indexierungsstrategie

1. **Vollindexierung**: Beim Hinzufügen eines Repos werden alle Archive
   via `borg list --json-lines <repo>::<archive>` eingelesen.
2. **Inkrementelle Indexierung**: Bei erneutem Index-Lauf werden nur neue
   Archive (id > last_indexed_archive_id) verarbeitet.
3. **Hintergrund**: Der Indexer läuft als Thread im Daemon oder als
   separater Timer-getriggerter Prozess.
4. **Größenordnung**: Der Pfad-Index kann bei großen Repos Millionen Zeilen
   umfassen. Batch-Inserts innerhalb einer Transaktion sind Pflicht.

## 3. DB-Portabilität

- Alle Timestamps als UTC, ISO-8601 TEXT (SQLite) bzw. `TIMESTAMPTZ` (PostgreSQL).
- UUIDs als TEXT (SQLite) bzw. native UUID (PostgreSQL).
- ENUM-Felder als TEXT mit CHECK-Constraint (SQLite) bzw. native ENUM (PostgreSQL).
- Migrationen als nummerierte `.sql`-Dateien unter `db/migrations/`.

---

**Dieses Dokument ist vor der ersten Implementierung vom Auftraggeber freizugeben.
Änderungen erfordern eine neue Version und erneute Freigabe.**
