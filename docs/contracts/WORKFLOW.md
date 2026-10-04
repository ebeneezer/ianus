# Ianus – UX-Workflow-Vertrag

Version: 0.1.0
Status: ENTWURF – freigabepflichtig

## 1. Zentrale Metapher

Ianus präsentiert dem Benutzer einen **Wald**: eine Menge von Bäumen, deren
Wurzeln die gesicherten Hosts/Repositories sind. Der Benutzer navigiert diesen
Wald wie ein Dateisystem – sieht aber stets den **aktuellsten Stand** jeder Datei
über alle Archive hinweg, ohne Archive manuell auswählen zu müssen.

## 2. Hauptbereiche der Oberfläche

```
┌─────────────────────────────────────────────────────────┐
│  Toolbar (Suche, Benutzer, Einstellungen)               │
├────────────────┬──────────────────────┬─────────────────┤
│                │                      │                 │
│  Waldbaum      │  Inhaltsbereich      │  Job-Dock       │
│  (linke        │  (Dateien des        │  (rechte Seite  │
│   Sidebar)     │   gewählten          │   oder unteres  │
│                │   Verzeichnisses)    │   Panel)        │
│  - Host A      │                      │                 │
│    └ /etc      │  [Datei] [Datei]     │  ┌───────────┐ │
│    └ /home     │  [Datei] [Datei]     │  │ Restore-  │ │
│  - Host B      │                      │  │ Job #1    │ │
│    └ /var      │                      │  │ 3 Objekte │ │
│                │                      │  └───────────┘ │
│                │                      │                 │
├────────────────┴──────────────────────┴─────────────────┤
│  Statusleiste (laufende Tasks, Indexierungsstatus)       │
└─────────────────────────────────────────────────────────┘
```

## 3. Interaktionen

### 3.1 Waldbaum-Navigation
- Baumknoten expandieren per Klick.
- Lazy-Loading: Kinder werden bei Expansion via API geladen.
- Suchfeld filtert den gesamten Baum (serverseitig, mit Debounce).

### 3.2 Rechtsklick → Kontextmenü
Auf jedes Objekt (Datei oder Verzeichnis):
- **Historie anzeigen**: Zeigt alle Versionen aus allen Archiven, chronologisch.
  Jede Version zeigt: Archivname, Datum, Größe, Änderungstyp (neu/geändert/gelöscht).
- **Zum Restore vormerken**: Fügt die ausgewählte Version dem aktiven Job hinzu.
- **Neuen Job erstellen**: Erstellt ein neues leeres Job-Objekt.
- **Eigenschaften**: Zeigt Metadaten (Rechte, Eigentümer, Größe, Hash).

### 3.3 Drag & Drop
- **Datei/Verzeichnis → Job-Dock**: Erstellt neuen Job oder fügt zum aktiven hinzu.
- **Datei/Verzeichnis → bestehendes Job-Objekt**: Fügt dem Job hinzu.
- **Job-Objekt → Ausführungszone**: Startet den Job (mit Bestätigungsdialog).
- **Task-Typ-Karten → Job-Objekt**: Ändert den Task-Typ eines Jobs
  (z. B. von "Restore" zu "Check").

### 3.4 Job-Dock
- Zeigt alle offenen Job-Objekte als Karten.
- Jede Karte zeigt: Typ, Anzahl Objekte, geschätzte Größe.
- Karten sind aufklappbar → zeigt die enthaltenen Pfade.
- Status: `Entwurf` → `In Warteschlange` → `Läuft` (mit Fortschrittsbalken) → `Fertig` / `Fehlgeschlagen`.
- Abgeschlossene Jobs bleiben in der Historie sichtbar.

### 3.5 Ausführungszone
- Ein definierter Bereich (z. B. unterer Rand oder spezielles Drop-Target),
  auf den man einen Job zieht um ihn zu starten.
- Vor Ausführung: Bestätigungsdialog mit Zusammenfassung (was, wohin, geschätzte Dauer).
- Bei Restore: Zielverzeichnis-Auswahl im Dialog.

## 4. Konfigurationsbereich

Erreichbar über Einstellungen-Icon in der Toolbar:
- **Repositories**: Liste aller Repo-Objekte, Hinzufügen/Bearbeiten/Löschen, Verbindungstest.
- **Benutzer & Gruppen**: CRUD, Rollenzuordnung.
- **Rollen**: Berechtigungen definieren, Scope festlegen.
- **Tasks**: Übersicht registrierter Task-Typen (read-only in Phase 1).
- **System**: Indexierungsintervall, Log-Level, DB-Backend, Daemon-Status.

## 5. Responsive Verhalten

- Primärziel: Desktop-Browser (≥ 1280px).
- Tablet (≥ 768px): Sidebar einklappbar, Job-Dock als unteres Panel.
- Mobil: Nicht im Kernscope, aber Layout bricht nicht.

## 6. Design-Sprache

- Inspiriert von CleanMyMac: heller Hintergrund, sanfte Schatten, abgerundete Karten.
- Farbschema: Neutrales Grau + ein Akzentton (vorläufig Blau/Teal, anpassbar).
- Icons: Lucide oder Phosphor (MIT-lizenziert).
- Typografie: System-Font-Stack (`-apple-system, BlinkMacSystemFont, "Segoe UI", ...`).
- Animationen: Dezent, max. 300ms, für Drag-Feedback und Panel-Übergänge.

---

**Dieses Dokument ist vor der ersten Implementierung vom Auftraggeber freizugeben.
Änderungen erfordern eine neue Version und erneute Freigabe.**
