# Ianus – Agent Instructions

## Before any implementation work

1. Read ALL contract documents under `docs/contracts/` first.
2. Verify the contracts are in status "FREIGEGEBEN" (approved).
   Do NOT implement against contracts in status "ENTWURF" (draft).
3. Present implementation proposals with churn estimates and risk assessment
   before writing code.

## Code conventions

- Backend: C99, POSIX. See `docs/contracts/CODESTYLE.md`.
- C indentation: tabs only, tab size 3; K&R brace style.
- Handwritten source files have a hard limit of 1000 characters, including whitespace and comments; refactor and split semantically into appropriately named files before exceeding it. Generated and vendored files are exempt.
- Prefer table-driven dispatch over long `if`/`else if` chains where it clarifies control flow.
- Prioritize efficiency, code reuse, and minimal RSS; avoid unnecessary abstractions.
- No Ianus identifier may start with an underscore; do not define or use reserved C identifiers.
- Naming: `ianus_<module>_<verb>()` for functions, `ianus_<name>_t` for types.
- Build system: CMake >= 3.20.
- All public functions require Doxygen comments.
- Every file needs an SPDX license header.

## Architecture

- See `docs/contracts/ARCHITECTURE.md` for the component overview.
- DB abstraction via `db_driver_t` function-pointer struct.
- Task framework via `ianus_task_t` registration interface.
- Borg interaction exclusively via subprocess + JSON parsing.

## Working language and communication

- Code, comments, commits: English.
- User-facing UI text, documentation, contract documents: German where specified.
- Agent responses to the maintainer: German; address the maintainer formally as "Sie" or "Dr. Raus".
- The maintainer is an experienced software engineer with multiple academic degrees in computer science and 45 years of programming experience across C, C++, Swift, Pascal, BASIC, assembly, and other languages. Omit trivial explanations and ask only substantive questions.
- Code-tranche approvals also authorize corrective changes within that tranche (transitively); do not request separate approval for in-scope corrections.
- The AI acts as the maintainer's advisor: explicitly identify risks and warnings, and offer relevant improvement recommendations instead of merely agreeing or implementing silently.
