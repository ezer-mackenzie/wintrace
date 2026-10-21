# Changelog

Notable changes to `wintrace` are documented here.

## [Unreleased]

### Planned

- Compatible fixes and maintenance updates.

## [0.7.0]

### Changed

- Run the API and Rust test suites on Linux to continuously validate portable
  EVTX/XML behavior and non-Windows platform gating.

## [0.6.0]

### Changed

- Run the Windows API and Rust test suite on both the minimum supported Python
  version (3.10) and the newest supported version (3.14).

## [0.5.0]

### Changed

- Type iterator results according to the selected output format across the
  module-level API and native file/log objects.

## [0.4.0]

### Changed

- Read `wintrace.__version__` from the compiled extension's Cargo package
  version, keeping the Python API and distribution metadata synchronized.

## [0.3.0]

### Fixed

- Preserve all values when an event contains repeated `EventData` names instead
  of silently keeping only the last value.

## [0.2.0]

### Fixed

- Live event iterators continue waiting when `EvtNext` reaches its polling
  timeout and stop only when the query reports that no more items are available.
- The extension stub no longer relies on the undeclared `typing_extensions`
  package for context-manager typing.

## [0.1.0]

### Added

- Live Windows Event Log channel queries with XPath filtering and forward or
  reverse record order.
- Offline EVTX file parsing with iterator and context-manager APIs.
- Record, dictionary, XML, and JSON result formats.
- Event metadata accessors and XML-to-dictionary/JSON helpers.
- Typed Python package surface, API documentation, and tests.
- Python 3.10 or newer is required.
