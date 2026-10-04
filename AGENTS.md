# Janus – Agent Instructions

## Before any implementation work

1. Read ALL contract documents under `docs/contracts/` first.
2. Verify the contracts are in status "FREIGEGEBEN" (approved).
   Do NOT implement against contracts in status "ENTWURF" (draft).
3. Present implementation proposals with churn estimates and risk assessment
   before writing code.

## Code conventions

- Backend: Rust (Edition 2024). See `docs/contracts/CODESTYLE.md`.
- Rust formatting: tabs only, tab size 3 (rustfmt.toml with hard_tabs).
- Handwritten source files have a soft limit of 1000 characters, including whitespace and comments; refactor and split semantically into appropriately named modules/files before exceeding it. Generated and vendored files are exempt.
- Prefer table-driven dispatch over long `if`/`else if` chains where it clarifies control flow.
- Prioritize efficiency, code reuse, and minimal RSS; avoid unnecessary abstractions.
- No Janus identifier may start with an underscore; no reserved-identifier look-alikes.
- Naming: modules `janus_<topic>`, types PascalCase, functions/fields snake_case.
- Build system: cargo; `Cargo.lock` is committed.
- All public items require Rustdoc comments.
- Every file needs an SPDX license header.

## Architecture

- See `docs/contracts/ARCHITECTURE.md` for the component overview.
- KV store abstraction via one canonical access API (no backend-specific calls outside adapters).
- Task framework via `JanusTask` trait registry.
- Borg interaction exclusively via subprocess + JSON parsing.

## Working language and communication

- Code, comments, commits: English.
- User-facing UI text, documentation, contract documents: German where specified.
- Agent responses to the maintainer: German; address the maintainer formally as "Sie" or "Dr. Raus".
- The maintainer is an experienced software engineer with multiple academic degrees in computer science and 45 years of programming experience across C, C++, Swift, Pascal, BASIC, assembly, and other languages. Omit trivial explanations and ask only substantive questions.
- Code-tranche approvals also authorize corrective changes within that tranche (transitively); do not request separate approval for in-scope corrections.
- The AI acts as the maintainer's advisor: explicitly identify risks and warnings, and offer relevant improvement recommendations instead of merely agreeing or implementing silently.
