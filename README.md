# wintrace

`wintrace` reads Windows Event Log channels and parses offline `.evtx` files
through a Python API backed by Rust and PyO3.

The `1.x` series provides a stable public API. Compatible additions and fixes
remain within this series; breaking changes are reserved for a new major
version. See [docs/api.md](docs/api.md) for the stability boundary.

## Installation

Requires Python 3.10 or newer.

```console
python -m pip install wintrace
```

Offline EVTX parsing works on supported platforms. Live channel queries require
Windows and the current user must have permission to read the selected channel.

## Read an EVTX file

```python
import wintrace

events = wintrace.read_evtx(r"C:\Logs\System.evtx", limit=20)
for event in events:
    print(event.time_created, event.event_id, event.provider)
```

For incremental processing:

```python
for event in wintrace.iter_evtx(r"C:\Logs\System.evtx"):
    print(event.event_id, event.computer)
```

`EvtxFile` can also be used as a context manager:

```python
with wintrace.EvtxFile(r"C:\Logs\System.evtx") as log:
    for event in log.iter():
        print(event.to_dict())
```

## Query live events on Windows

```python
events = wintrace.query_events(
    channel="System",
    query="*[System[(Level <= 3)]]",
    limit=10,
)
```

`iter_events()` streams results from the selected channel. `LiveEventLog`
provides the same operations as an object.

## Output formats

Readers accept `format="record"` (default), `"dict"`, `"xml"`, or `"json"`.
`EventRecord` provides event metadata, `to_dict()`, `to_json()`, `get()`, and
dictionary-style access.

XML conversion helpers are available as `wintrace.xml_to_dict(xml)` and
`wintrace.xml_to_json(xml)`.

## Development

Build and install the local package with Maturin, then run the tests:

```console
maturin develop
python -m unittest discover -s tests
cargo test --no-default-features
```

See [docs/api.md](docs/api.md) for signatures and
[docs/architecture.md](docs/architecture.md) for the Python/Rust boundary.
