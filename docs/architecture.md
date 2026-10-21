# Architecture

```text
Python application
        |
        v
Public package: wintrace
        |
        v
Private PyO3 extension: wintrace._wintrace
        |
        +-- EVTX reader (evtx crate)
        +-- Windows channel reader (Windows Event Log API)
        +-- Event XML parsing and record conversion
```

The Python package exposes the stable API and type information. Rust performs
EVTX record reading, event XML conversion, and (on Windows) channel queries.
Platform-specific Event Log calls are conditionally compiled; offline EVTX
parsing and XML utilities remain available on other supported platforms.

EVTX iterators move the file parser to a reader thread and return records through
a bounded-by-consumption channel, so callers do not need to first materialize
the complete input file.
