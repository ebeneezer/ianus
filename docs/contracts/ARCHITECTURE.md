# Ianus – Architekturvertrag

Version: 0.1.0
Status: ENTWURF – freigabepflichtig

## 1. Überblick

Ianus ist ein Web-UI für Borg Backup, das einen intuitiven, Drag-&-Drop-basierten
Workflow auf Linux-systemd-Systemen bereitstellt. Das Design orientiert sich an
CleanMyMac: aufgeräumt, visuell klar, interaktionsgetrieben.

## 2. Systemkomponenten

```
┌─────────────────────────────────────────────────┐
│  Browser (SPA)                                  │
│  ┌──────────┐ ┌──────────┐ ┌──────────────────┐│
│  │ Waldbaum  │ │ Job-Dock │ │ Konfig-Explorer  ││
│  │ (Explorer)│ │ (D&D)    │ │ (RBAC, Repos)    ││
│  └──────────┘ └──────────┘ └──────────────────┘│
└───────────────────┬─────────────────────────────┘
                    │ REST + WebSocket
┌───────────────────┴─────────────────────────────┐
│  ianus-daemon (C)                               │
│  ┌──────────┐ ┌──────────┐ ┌──────────────────┐│
│  │ HTTP/WS  │ │ Indexer   │ │ Task-Runner      ││
│  │ Server   │ │ (Cache)   │ │ (Borg-Subproz.)  ││
│  └──────────┘ └──────────┘ └──────────────────┘│
│  ┌──────────────────────────────────────────────┐│
│  │ DB-Abstraktionsschicht (sqlite / pgsql)      ││
│  └──────────────────────────────────────────────┘│
└─────────────────────────────────────────────────┘
        │                         │
   Borg CLI (JSON)          Datenbank
   (lokal/ssh/borg-serve)   (SQLite/PostgreSQL)
```

## 3. Backend: ianus-daemon

- **Sprache**: ausschließlich C (C17, POSIX).
- **HTTP/WebSocket-Server**: civetweb (eingebettet, lizenzkompatibel MIT).
- **JSON**: cJSON (eingebettet).
- **Borg-Integration**: Subprozess-Steuerung (`posix_spawn` / `fork+exec`),
  Parsen von `--json` / `--json-lines`-Ausgaben. Kein Python-Embedding.
- **Secrets**: `systemd-creds` oder `BORG_PASSCOMMAND`; niemals Klartext in
  Konfigurationsobjekten.
- **Build**: CMake (>= 3.20), keine Autotools.

## 4. Repo-Zugriffsmodell

Ein `Repository`-Objekt kapselt alles für den Zugriff auf ein Borg-Repo:

| Transporttyp   | Konfigurationsfelder                             |
|----------------|--------------------------------------------------|
| Lokal          | `path`                                           |
| SSH            | `ssh_host`, `ssh_user`, `ssh_port`, `path`       |
| SFTP           | `sftp_host`, `sftp_user`, `sftp_port`, `path`    |
| Borg-Server    | `borg_host`, `borg_user`, `borg_port`, `path`    |

Zugriffskonfigurationen werden in der DB gespeichert; Passphrasen und
SSH-Schlüssel werden über referenzierte Secret-IDs (systemd-creds oder
verschlüsselt in der DB mit einem Master-Key) aufgelöst, nie inline.

## 5. Datenbank-Abstraktionsschicht

- Interface: `db_driver_t` struct mit Funktionspointern
  (`open`, `close`, `exec`, `prepare`, `step`, `finalize`, `begin`, `commit`, `rollback`).
- Referenztreiber Phase 1: **SQLite** (libsqlite3), **PostgreSQL** (libpq).
- Schema ist SQL-portabel (keine DB-spezifischen Typen im Kernpfad).
- MariaDB-Treiber als designierte Phase-2-Erweiterung.
- MongoDB wird nur bei explizitem Bedarf als Audit-/Event-Log-Adapter ergänzt,
  nicht für den Kernbestand.

## 6. Frontend (SPA)

- Svelte (oder React – Entscheidung in FRONTEND.md).
- Drag & Drop: dnd-kit oder Svelte-natives D&D.
- Virtualisierter Baumexplorer (Lazy-Load bei > 1000 Einträgen pro Verzeichnis).
- Design-Sprache: CleanMyMac-inspiriert – helles, aufgeräumtes UI, Karten-Metapher
  für Jobs, sanfte Animationen.
- WebSocket für Echtzeit-Fortschritt laufender Tasks.

## 7. systemd-Integration

- `ianus.service`: startet `ianus-daemon`, Type=notify, Restart=on-failure.
- Optional: `ianus-indexer.timer` für periodische Repo-Indexierung.
- Socket-Activation möglich (Phase 2).

## 8. Erweiterbarkeit: Task-Framework

Jeder Task ist ein eigenständiger Handler mit definiertem Interface:

```c
typedef struct ianus_task {
    const char *name;
    const char *description;
    ianus_task_input_type_t input_type;  // PATHS, REPO, ARCHIVE, ...
    ianus_role_t required_role;
    int (*validate)(const ianus_task_ctx_t *ctx);
    int (*execute)(const ianus_task_ctx_t *ctx, ianus_progress_cb progress);
    int (*cancel)(const ianus_task_ctx_t *ctx);
} ianus_task_t;
```

Phase 1: `restore`, `check`.
Phase 2: `prune`, `compact`, `mount-readonly`, `create` (neuer Snapshot).
Weitere Tasks werden über dasselbe Interface registriert.

## 9. Konfigurationsobjekt

Ein zentrales, persistiertes Objekt mit Unterobjekten:

- `repos[]` – Repository-Definitionen
- `users[]` – Benutzer (Passwort-Hash, Rollen-Zuordnung)
- `groups[]` – Gruppen
- `roles[]` – Rollen mit Berechtigungen (`repo:read`, `restore:execute`, `task:prune`, `admin:config`, …)
- `tasks[]` – Registrierte Task-Typen
- `jobs[]` – Job-Instanzen und -Historie

Im Web-UI wird dieses Objekt als navigierbarer Teilbaum dargestellt.

## 10. Sicherheit

- Kein Klartext-Passwort in Konfiguration oder Datenbank.
- HTTPS per Reverse-Proxy oder eingebettetem TLS (civetweb unterstützt beides).
- Session-basierte Authentifizierung (Token, httpOnly-Cookie).
- RBAC wird bei jedem API-Aufruf geprüft, nicht nur im Frontend.
- Rate-Limiting auf Login-Endpunkt.

---

**Dieses Dokument ist vor der ersten Implementierung vom Auftraggeber freizugeben.
Änderungen erfordern eine neue Version und erneute Freigabe.**
