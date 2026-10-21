# API reference

The public API is exported from `wintrace`.

## Live event queries

```python
query_events(
    channel="System",
    query="*",
    limit=None,
    reverse=True,
    format="record",
)
iter_events(channel="System", query="*", reverse=True, format="record")
```

These functions query a local Windows Event Log channel. `query` is an XPath
filter. `limit=None` returns all matching records. Live queries are unavailable
on non-Windows systems.

`LiveEventLog(channel="System", query="*", reverse=True)` offers `read()` and
`iter()` methods with the same filtering and output options.

## Offline EVTX parsing

```python
read_evtx(path, limit=None, format="record")
iter_evtx(path, format="record")
```

`EvtxFile(path)` provides `read()` and `iter()` and supports the `with`
statement. Iterators yield records on demand. `read_evtx()` returns a list and
can be bounded with `limit`.

## Output formats

`format` accepts `"record"`, `"dict"`, `"xml"`, or `"json"`. The default
`"record"` returns `EventRecord` objects. Each record includes `xml`,
`event_id`, `record_id`, `channel`, `provider`, `level`, `time_created`,
`computer`, `system`, and `event_data`. It also provides `to_dict()`,
`to_json()`, `get(key, default=None)`, and indexing by event section.

For `event_data`, named `<Data>` elements are keyed by their `Name` attribute.
When a name appears more than once, its value is a list preserving document
order. Unnamed data elements are retained under the `@unnamed` key when named
elements are also present.

## XML helpers

- `xml_to_dict(xml)` parses Windows Event XML into nested Python dictionaries.
- `xml_to_json(xml)` returns a formatted JSON representation.
