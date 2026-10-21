"""Windows Event Log queries and EVTX parsing backed by Rust."""

from ._wintrace import (
    __version__,
    EventRecord,
    EvtxFile,
    EvtxRecordIterator,
    LiveEventIterator,
    LiveEventLog,
)
from .live import iter_events, query_events
from .offline import iter_evtx, read_evtx
from .utils import xml_to_dict, xml_to_json

__all__ = [
    "__version__",
    "EventRecord",
    "EvtxFile",
    "EvtxRecordIterator",
    "LiveEventIterator",
    "LiveEventLog",
    "iter_events",
    "iter_evtx",
    "query_events",
    "read_evtx",
    "xml_to_dict",
    "xml_to_json",
]
