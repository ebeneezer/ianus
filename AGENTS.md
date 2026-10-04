# Ianus – Agent Instructions

## Before any implementation work

1. Read ALL contract documents under `docs/contracts/` first.
2. Verify the contracts are in status "FREIGEGEBEN" (approved).
   Do NOT implement against contracts in status "ENTWURF" (draft).
3. Present implementation proposals with churn estimates and risk assessment
   before writing code.

## Code conventions

- Backend: C17, POSIX. See `docs/contracts/CODESTYLE.md`.
- Naming: `ianus_<module>_<verb>()` for functions, `ianus_<name>_t` for types.
- Build system: CMake >= 3.20.
- All public functions require Doxygen comments.
- Every file needs an SPDX license header.

## Architecture

- See `docs/contracts/ARCHITECTURE.md` for the component overview.
- DB abstraction via `db_driver_t` function-pointer struct.
- Task framework via `ianus_task_t` registration interface.
- Borg interaction exclusively via subprocess + JSON parsing.

## Working language

- Code, comments, commits: English.
- User-facing UI text, documentation, contract documents: German where specified.
- Agent responses to the user: German.
