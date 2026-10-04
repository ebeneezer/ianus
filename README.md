# Janus

A modern, drag-and-drop Web UI for [Borg Backup](https://www.borgbackup.org/),
inspired by CleanMyMac's design language. Built for Linux systemd systems.

## Concept

Janus presents all Borg archives as a **unified filesystem tree** (a "forest"),
always showing the latest version of every file. Right-click any object to
browse its version history. Drag files onto Job objects to compose restore
operations — or any other Borg task — before executing them.

## Architecture

- **Backend**: C daemon (`janus-daemon`) with embedded HTTP/WebSocket server
- **Frontend**: SPA with drag-and-drop workflow
- **Database**: Pluggable backend (SQLite for local, PostgreSQL for multi-user)
- **Borg integration**: Subprocess control, JSON output parsing
- **Deployment**: systemd service unit

## Status

🚧 Pre-alpha — design contracts under review.

## Documentation

Design contracts (binding, versioned):

- [Architecture](docs/contracts/ARCHITECTURE.md)
- [Data Model](docs/contracts/DATAMODEL.md)
- [Code Style](docs/contracts/CODESTYLE.md)
- [UX Workflow](docs/contracts/WORKFLOW.md)
- [Task Framework](docs/contracts/TASKFRAMEWORK.md)

## License

[GPL-3.0-or-later](LICENSE)
