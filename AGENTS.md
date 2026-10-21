# Project Guidelines

## Architecture

- Rust/PyO3 implements the extension in `src/`; the Python package facade is
  under `python/wintrace/`.
- Keep the public Python wrappers and `_wintrace.pyi` stub aligned with the
  extension API.
- Live Windows Event Log queries are platform-specific; EVTX and XML parsing
  should remain portable.
- The minimum supported Python version is 3.10. See
  [docs/architecture.md](docs/architecture.md) for component details.

## Build and Test

Use the commands and setup in [CONTRIBUTING.md](CONTRIBUTING.md). At minimum,
run formatting, Rust tests, and the Python unittest suite for relevant changes.
Live-query integration tests require Windows.

## Changes

- Add or update tests for behavior changes.
- Update API documentation and `CHANGELOG.md` when user-visible behavior
  changes.
- Do not commit generated binaries, private event logs, or credentials.