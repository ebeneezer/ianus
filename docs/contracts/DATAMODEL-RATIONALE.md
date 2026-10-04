# Ianus – Datenmodell-Rationale: SQL vs. MongoDB

Version: 0.1.0
Status: ENTWURF – freigabepflichtig

## Frage

Macht MongoDB für Ianus Sinn? Liefert Borg JSON-Objekte, die gut zu MongoDB passen?

## Antwort: Nein für Phase 1, optional für Phase 2+

### 1. Was liefert Borg als JSON?

Borg unterstützt mehrere JSON-Ausgabeformate:

#### `borg info --json`
Ein großes Objekt mit:
```json
{
  "repository": { "id": "...", "location": "...", "last_modified": "..." },
  "encryption": { "mode": "..." },
  "cache": { "total_chunks": 123, "unique_chunks": 45, ... },
  "archives": [
    {
      "name": "hostname-2026-10-04T12:00:00",
      "id": "abc123...",
      "start": "2026-10-04T12:00:00",
      "end": "2026-10-04T12:05:00",
      "duration": 300,
      "stats": { "original_size": 1000000, "compressed_size": 500000, "deduplicated_size": 250000 },
      "nfiles": 5000,
      "hostname": "myhost",
      "username": "root",
      "command_line": "borg create ..."
    }
  ]
}
```

#### `borg list --json` (Archive)
```json
{
  "archives": [
    { "name": "...", "id": "...", "start": "...", "end": "...", ... }
  ]
}
```

#### `borg list --json-lines` (Dateien)
Streaming, jede Zeile ein Objekt:
```json
{"type": "file", "mode": "0644", "user": "root", "group": "root", "uid": 0, "gid": 0, "path": "/etc/passwd", "size": 1234, "mtime": "2026-10-04T10:00:00", "isomtime": "2026-10-04T10:00:00.000000"}
{"type": "dir", "mode": "0755", "user": "root", "group": "root", "uid": 0, "gid": 0, "path": "/home", "size": 0, "mtime": "2026-10-04T10:00:00", "isomtime": "2026-10-04T10:00:00.000000"}
```

#### `borg create --log-json` (Progress)
```json
{"type": "archive_progress", "original_size": 1000000, "compressed_size": 500000, "deduplicated_size": 250000, "nfiles": 5000, "path": "/home/user/file.txt"}
{"type": "file_status", "status": "A", "path": "/home/user/newfile.txt"}
{"type": "log_message", "time": "2026-10-04T12:00:00", "levelname": "INFO", "name": "borg.create", "message": "..."}
```

### 2. Oberflächlich: Ja, Borg liefert JSON

MongoDB speichert JSON-Dokumente. Borg liefert JSON. Auf den ersten Blick passt das zusammen.

**Aber:** Das ist ein Trugschluss. Die Frage ist nicht "liefert Borg JSON?", sondern "passt das Datenmodell zu MongoDB?"

### 3. Das Kernproblem: Ianus braucht einen **Merged Filesystem Tree**

Ianus' zentrale Anforderung ist, einen **einheitlichen Dateisystem-Baum** über alle Archive hinweg zu zeigen, mit stets der **aktuellsten Version** jeder Datei.

Das bedeutet:
- Für jeden Pfad `/etc/passwd` über alle Archive hinweg
- Finde die Version mit dem neuesten `mtime`
- Zeige diese an (nicht alle Versionen)

**SQL-Lösung** (effizient):
```sql
SELECT DISTINCT ON (repo_id, path)
       repo_id, path, archive_id, mtime, size, mode, ...
FROM   paths
ORDER  BY repo_id, path, mtime DESC;
```

**MongoDB-Lösung** (ineffizient):
```javascript
db.paths.aggregate([
  { $group: {
      _id: { repo_id: "$repo_id", path: "$path" },
      latest: { $max: "$mtime" },
      doc: { $first: "$$ROOT" }
    }
  },
  { $replaceRoot: { newRoot: "$doc" } }
])
```

Das ist langsamer, speicherintensiver und schwerer zu optimieren.

### 4. Strukturelle Probleme mit MongoDB

#### 4.1 Keine echten Fremdschlüssel
Ianus braucht:
- `repos` → `archives` (1:N)
- `archives` → `paths` (1:N)
- `users` → `roles` (M:N über `user_roles`)

MongoDB hat keine echten Fremdschlüssel. Wenn man ein Archiv löscht, muss man manuell alle zugehörigen `paths`-Dokumente löschen. Das führt zu Dateninkonsistenzen.

SQL mit `FOREIGN KEY ... ON DELETE CASCADE` macht das automatisch.

#### 4.2 Keine Transaktionen über Dokumente hinweg
Wenn man einen Job erstellt und mehrere `job_items` hinzufügt, braucht man eine Transaktion:
- Entweder alle `job_items` werden eingefügt, oder keine.

MongoDB 4.0+ hat Multi-Document-Transaktionen, aber:
- Sie sind langsamer als SQL-Transaktionen
- Sie sind auf Replica Sets beschränkt (nicht auf Standalone)
- Sie sind komplizierter zu debuggen

#### 4.3 Keine effizienten Indizes für komplexe Queries
Ianus braucht Indizes wie:
- `(repo_id, path, mtime DESC)` – für "aktuellste Version pro Pfad"
- `(repo_id, parent_path)` – für Verzeichnis-Listing
- `(archive_id)` – für Archiv-bezogene Abfragen

MongoDB kann das, aber SQL ist hier natürlicher und effizienter.

### 5. Performance-Vergleich

Für Ianus' Workload (Millionen von Pfaden, komplexe Queries):

| Aspekt | PostgreSQL | SQLite | MongoDB |
|--------|-----------|--------|---------|
| **Schreibdurchsatz** | Sehr hoch | Mittel (Single-Writer) | Hoch, aber langsamer bei Transaktionen |
| **Komplexe Queries** | Sehr schnell (SQL) | Schnell | Langsam (Aggregation-Framework) |
| **Speichereffizienz** | Sehr gut | Sehr gut | Schlecht (55% mehr Speicher) |
| **Fremdschlüssel** | Nativ | Nativ | Manuell |
| **Transaktionen** | ACID, Multi-Row | ACID, Single-Writer | Multi-Doc, aber kompliziert |
| **Skalierbarkeit** | Horizontal (Sharding) | Lokal nur | Horizontal (Sharding) |

**Fazit:** Für Ianus' Kernbestand (repos, archives, paths, jobs) ist SQL überlegen.

### 6. Wann könnte MongoDB Sinn machen?

#### Phase 2+: Audit-Log / Event-Streaming
Wenn Ianus später ein Audit-Log braucht (wer hat was wann getan), könnte MongoDB sinnvoll sein:
- Jedes Event ist ein unabhängiges Dokument
- Keine Fremdschlüssel nötig
- Flexible Schema (neue Event-Typen ohne Migration)
- Gute Unterstützung für TTL-Indizes (alte Events automatisch löschen)

**Aber:** PostgreSQL mit JSONB ist auch hier eine gute Alternative:
```sql
CREATE TABLE audit_log (
  id BIGSERIAL PRIMARY KEY,
  event_type TEXT,
  event_data JSONB,
  created_at TIMESTAMP DEFAULT NOW()
);
CREATE INDEX idx_audit_created ON audit_log(created_at DESC);
```

Das ist einfacher zu betreiben (eine DB statt zwei) und genauso flexibel.

### 7. Empfehlung

#### Phase 1 (MVP)
- **Primär:** SQLite (lokal) oder PostgreSQL (Multi-User)
- **Nicht:** MongoDB
- **Begründung:** Das Datenmodell ist relational. MongoDB würde Komplexität ohne Vorteil bringen.

#### Phase 2+
- **Optional:** MongoDB als separates Audit-Log-Backend
- **Oder:** PostgreSQL mit JSONB (einfacher, eine DB)
- **Entscheidung:** Später, basierend auf Anforderungen

### 8. Zusammenfassung

| Frage | Antwort |
|-------|---------|
| Liefert Borg JSON? | Ja, aber das ist nicht das Kriterium. |
| Passt MongoDB zu Borg-JSON? | Oberflächlich ja, aber strukturell nein. |
| Macht MongoDB für Ianus Sinn? | Nein für Phase 1. Optional für Phase 2+ (Audit-Log). |
| Was ist besser? | SQL (PostgreSQL/SQLite) für Kernbestand. |

---

**Dieses Dokument ist vor der ersten Implementierung vom Auftraggeber freizugeben.
Änderungen erfordern eine neue Version und erneute Freigabe.**
