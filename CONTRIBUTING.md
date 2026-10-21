# Contributing

Contributions to `wintrace` are welcome. For substantial changes, open an issue
first to discuss the proposed behavior or API.

## Development Setup

The project requires Python 3.10 or newer, Rust 1.88 or newer, and Maturin.
From the repository root, build the extension into the active Python
environment:

```console
python -m pip install maturin
maturin develop
```

## Before Submitting

Run the checks relevant to your change:

```console
cargo fmt -- --check
cargo clippy --all-targets --no-default-features -- -D warnings
cargo test --no-default-features
python -m unittest discover -s tests -v
```

Live Event Log queries require Windows. EVTX and XML behavior should remain
portable, so run the Rust and Python tests on other supported platforms when
possible. On Windows, the Rust test executable may need the selected Python
installation's directory on `PATH` so it can load the Python runtime DLL.

Keep public Python wrappers, overloads, and `python/wintrace/_wintrace.pyi`
consistent with the PyO3 API. Add or update tests and user-facing
documentation when changing behavior.

## Pull Requests

- Keep changes focused and explain the user-visible effect.
- Include tests for new behavior and bug fixes.
- Document compatibility changes and update `CHANGELOG.md` when appropriate.
- Do not include credentials, private event logs, generated binaries, or
  unrelated local files.

By participating, you agree to follow the project's
[Code of Conduct](CODE_OF_CONDUCT.md).