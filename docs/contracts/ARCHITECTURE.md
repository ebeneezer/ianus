# Janus – Architekturvertrag

Version: 0.3.1
Status: FREIGEGEBEN

## 1. Überblick

Janus ist ein Web-UI für Borg Backup, das einen intuitiven, Drag-&-Drop-basierten
Workflow auf Linux-systemd-Systemen bereitstellt. Das Design orientiert sich an
CleanMyMac: aufgeräumt, visuell klar, interaktionsgetrieben.

Produktziel: Janus macht Borg angenehm, verständlich und vertrauenswürdig – für
das Anlegen von Backups, die Erkundung von Archiven und die Wiederherstellung.
Borg ist häufig schweigsam und Remote-Speicher kann langsam sein; Janus macht
diese Arbeit transparent und nachvollziehbar: laufendes, verständliches Feedback
zu Fortschritt, voraussichtlicher Dauer/Restzeit und sauberem Cancel. Dabei wird
keine erfundene Gewissheit oder erfundener Fortschritt suggeriert.

Die Fortschritts-Feedback-Architektur läuft über WebSocket: Tasks melden Phasen,
Zähler und Status; Cancel-Anfragen gehen vom Frontend an den Daemon und werden an
Task-Runner, DB-Adapter und Borg-Subprozess weitergegeben. Nicht jede zugrunde
liegende Borg-/DB-Operation ist sofort abbrechbar; der Vertrag verspricht keinen
sofortigen Abbruch, sondern einen sauberen, nachvollziehbaren Ablauf.

**Abbruch-Eskalationsleiter für Borg-Subprozesse** (Entscheidung des
Auftraggebers): Cancel-Anfrage → SIGTERM an die Prozessgruppe des Kindes →
kurze Grace-Periode → SIGKILL (letzte Instanz, „der Hammer"). Ein per SIGKILL
beendetes Borg kann stale Repository-Locks hinterlassen; der Daemon meldet
dies offen und bietet die Lock-Bereinigung an (`break-lock`, Task-Katalog).
Eigene asynchrone Daemon-Arbeit (DB-Transaktionen, Stream-Parsing) bricht an
definierten Abbruchpunkten kooperativ ab; SQLite-Transaktionen rollen
zurück. Der Hammer ist demnach die Ausnahme, nicht das Mittel der Wahl.

## 2. Systemkomponenten

```mermaid
flowchart TB
    subgraph Browser["Browser (SPA)"]
        Waldbaum["Waldbaum (Explorer)"]
        JobDock["Job-Dock (D&D)"]
        Konfig["Konfig-Explorer (RBAC, Repos)"]
    end

    Browser -- "REST + WebSocket" --> Daemon

    subgraph Daemon["janusd (Rust)"]
        HTTP["HTTP/WS Server"]
        Indexer["Indexer (Cache)"]
        Runner["Task-Runner (Borg-Subprozess)"]
        KVAPI["KV Access API (kanonisch)"]
        DBAdapter["DB-Adapter (sqlite / pgsql)"]
    end

    Daemon --> BorgCLI["Borg CLI (JSON)\n(lokal/ssh/borg-serve)"]
    KVAPI --> DBAdapter
    DBAdapter --> DB["Datenbank\n(SQLite/PostgreSQL)"]
```

## 3. Backend: janusd

- **Sprache**: Rust (Edition 2024) – Entscheidung des Auftraggebers im
  Architektur-Review: memory-safe by design für den Daemon, der unvetrauens-
  wuerdige Eingaben parst (Borg-JSONL-Streams, HTTP/WS-Input). Exklusiv-
  Entscheidungsgrund war nicht Performance, sondern Safety by Design;
  Leistung war ohnehin nie das dominante Argument. Der Daemon bleibt ein
  schlankes systemd-Binary ohne GC; das Projekt ist ein agentic-coding-
  Projekt (AI-generierter Code, menschliche Pruefung durch den
  Maintainer), wofuer Rusts compile-time-Safety den Review-Aufwand senkt.
- **HTTP/WebSocket-Server**: axum (MIT/Apache-2.0, tokio-basiert) –
  produktionsreifes, weitverbreitetes Ökosystem; TLS via rustls.
- **JSON**: serde_json für einzelne begrenzte Objekte; Borg-JSONL wird
  inkrementell/streamend geparst (serde `StreamDeserializer`-Prinzip oder
  handgefilterter NDJSON-Reader); eine ganze große Borg-Antwort wird niemals
  vollständig als DOM im Speicher gehalten.
- **Borg-Integration**: Subprozess-Steuerung (`std::process::Command`,
  asynchron via tokio), Parsen von `--json` / `--json-lines`-Ausgaben.
  Kein Python-Embedding.
- **Secrets**: `systemd-creds` oder `BORG_PASSCOMMAND`; niemals Klartext in
  Konfigurationsobjekten.
- **Build**: cargo (Rust-Standard); CMake entfällt.

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

## 5. Persistenz: KV Access API und Backend-Adapter

Die Persistenz ist zweistufig aufgebaut:

1. **KV Access API (kanonisch)**: eine einzige C-Key/Value-Zugriffs-API
   (`get`, `put`, `delete`, `prefix-scan` mit begrenzten Seiten/Cursor sowie
   Batch-/Transaktionssemantik), die von **allen Fachmodulen** verwendet wird.
   Kein Fachmodul darf direkt Backend-spezifische DB-Aufrufe nutzen.
2. **Backend-Adapter**: SQLite (rusqlite, gebündelt) und PostgreSQL (tokio-postgres) als
   austauschbare persistente Backends hinter Adaptern der API. MariaDB als
   designierte Phase-2-Erweiterung. MongoDB wird nur bei explizitem Bedarf als
   Audit-/Event-Log-Adapter ergänzt, nicht für den Kernbestand.

Die API ist klar debugbar: ein zentraler Ort für strukturierte Operationslogs
und Metriken (Key/Namespace, Resultatgröße, Fehler). JSON-Payloads und Secrets
werden niemals geloggt. Es gibt keinen DB-eigenen zweiten Cache; der alleinige
persistente Wahrheitsbestand ist das KV-Backend.

Die **Janus-Eigenkonfiguration** (Color-Theme, UI-Sprache, künftige
Einstellungen) liegt in einem **eigenen lokalen SQLite-Speicher** hinter
derselben KV Access API: Der Daemon bleibt auch bei unerreichbarem
Remote-Backend bootstrap- und konfigurierbar. Eigenkonfiguration und
Indexbestand sind disjunkte Datenmengen – keine doppelte Datenhaltung.

## 6. Runtime Host Cache

Der Runtime Host Cache ist ein **volatiler, process-lokaler** Cache im Daemon:

- Er ist explizit **kein zweiter Persistenzbestand** und **kein DB-Cache**;
  der alleinige persistente Wahrheitsbestand bleibt das KV-Backend.
- Beim Öffnen eines Hosts/Repos werden die Daten über die Store API
  **on-demand und seitenweise** geladen und in einer C-Struktur mit für
  Filterung und Jobsteuerung geeigneten Indizes gehalten.
- Ein offener Host/Repo hält einen **Lease** auf den Cache. `host close`
  startet eine **Grace-Periode**; die tatsächliche Eviction erfolgt erst,
  wenn die Frist abgelaufen ist und kein anderer UI- oder Job-Lease besteht.
- **Job-Pins** bleiben stabil: Sie referenzieren persistente Store-IDs und
  müssen nach einer Eviction aus dem KV-Backend wiederaufladbar sein.
- Der Cache ist durch ein explizites RSS-/Speicherbudget begrenzt und
  evictbar; ein Cache-Miss liest über dieselbe kanonische Store API nach.
- **Warnung**: Der vollständige, unbegrenzte Eager-Load eines
  Millionen-Zeilen-Pfadbestands ist zu vermeiden (RSS-Risiko); geladen wird
  on-demand und seitenweise.

## 7. Frontend (SPA)

- **Stack (verbindlich)**: Svelte 5 + Vite + TypeScript. Entscheidung des
  Auftraggebers gegen React/Vue/Vanilla begründet: komponentenfeingranulare
  Reaktivität ohne VDOM (Live-Task-Ticker, große lazy Trees), eingebaute
  Transition-Engine für die CleanMyMac-Anmutung (`prefers-reduced-motion`
  ist Idiom, nicht Nachbau), kleine Runtime, minimaler Dependency-Churn.
- **JS-Quarantäne (verbindlich)**: JavaScript existiert ausschließlich als
  Build-Artifact. Handgeschriebene `.js`-Dateien sind im Frontend-Quellbaum
  verboten und per Linter/CI zu erzwingen; geschrieben wird nur `.svelte`
  und `.ts` (TypeScript `strict`, `any` verboten). Der Daemon, alle
  Build-Skripte für Artefakte und jede Logik außerhalb des Browsers bleiben
  reines Rust im Daemon. WASM/C-Routen für das Frontend sind geprüft und
  bewusst verworfen: kein direkter
  DOM-Zugriff ohne JS-Klebstoff, kein reifes Komponenten-/DnD-Ökosystem,
  geschätzter Mehrfachaufwand für das interaktionsgetriebene UI.
- Kein UI-Kit (MUI/Chakra/Bootstrap): das individuelle Objekt-Design wird
  als eigene Primitives gebaut – CSS-Custom-Properties mit der verbindlichen
  Palette aus WORKFLOW §6.
- Drag & Drop: native HTML5-DnD-API mit eigenen MIME-Typen
  (`application/janus-host`, `application/janus-object`,
  `application/janus-task`) für den Objektfluss zwischen Panes;
  `svelte-dnd-action` (MIT) nur falls Listen-Sortierung gebraucht wird.
- Virtualisierter Baumexplorer (Lazy-Load bei > 1000 Einträgen pro Verzeichnis).
- Design-Sprache: CleanMyMac-inspiriert – helles, aufgeräumtes UI, Objekt-Metapher
  (abgerundete Rechtecke), sanfte Animationen max. 300 ms.
- WebSocket für Echtzeit-Fortschritt laufender Tasks.
- UI-Sprache und Farbtheme sind einstellbar: Themes als CSS-Custom-Properties
  (Laufzeitumschaltung ohne Neustart), Übersetzungen als statische Bundles;
  im Konfig-Speicher liegen nur die gewählten Werte.
- Frontend-Build (`vite build`) erzeugt statische Assets, die janusd direkt
  ausliefert – kein separater Node-Server im Betrieb.

## 8. systemd-Integration

- `janus.service`: startet `janusd`, Type=notify, Restart=on-failure.
- Optional: `janus-indexer.timer` für periodische Repo-Indexierung.
- Socket-Activation möglich (Phase 2).

## 9. Erweiterbarkeit: Task-Framework

Jeder Task ist ein eigenständiger Handler mit definiertem Interface
(skizziert; verbindliche Ausgestaltung im Task-Framework-Vertrag):

```rust
pub trait JanusTask: Send + Sync {
	fn name(&self) -> &'static str;
	fn input_type(&self) -> TaskInput;          // Paths, Repo, Archive, ...
	fn required_role(&self) -> Role;
	fn validate(&self, ctx: &TaskCtx) -> Result<(), TaskError>;
	async fn execute(&self, ctx: TaskCtx, progress: ProgressSink) -> Result<(), TaskError>;
	async fn cancel(&self, ctx: &TaskCtx) -> Result<(), TaskError>;
}
```

Registrierung tabellarisch (`registry: &[TaskSpec]`), kein `if/else`-Dispatch.

Phase 1: `restore`, `check`.
Phase 2: `prune`, `compact`, `mount-readonly`, `create` (neuer Snapshot).
Weitere Tasks werden über dasselbe Interface registriert.

## 10. Konfigurationsobjekt

Ein zentrales, persistiertes Objekt mit Unterobjekten:

- `repos[]` – Repository-Definitionen
- `users[]` – Benutzer (Passwort-Hash, Rollen-Zuordnung)
- `groups[]` – Gruppen
- `roles[]` – Rollen mit Berechtigungen (`repo:read`, `restore:execute`, `task:prune`, `admin:config`, …)
- `tasks[]` – Registrierte Task-Typen
- `jobs[]` – Job-Instanzen und -Historie
- `cfg[]` – Janus-Eigenkonfiguration: Color-Theme, UI-Sprache, künftige
  Einstellungen (eigener lokaler SQLite-Speicher, siehe §5)

Im Web-UI wird dieses Objekt als navigierbarer Teilbaum dargestellt.

## 11. Sicherheit

- Kein Klartext-Passwort in Konfiguration oder Datenbank.
- HTTPS per Reverse-Proxy oder eingebettetem TLS (axum/rustls unterstützt beides).
- Session-basierte Authentifizierung (Token, httpOnly-Cookie).
- RBAC wird bei jedem API-Aufruf geprüft, nicht nur im Frontend.
- Rate-Limiting auf Login-Endpunkt.

---

**Dieses Dokument ist vor der ersten Implementierung vom Auftraggeber freizugeben.
Änderungen erfordern eine neue Version und erneute Freigabe.**
