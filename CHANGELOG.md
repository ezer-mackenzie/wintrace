# Changelog

Notable changes to `wintrace` are documented here.

## [Unreleased]

### Planned

- Compatible fixes and maintenance updates.

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
