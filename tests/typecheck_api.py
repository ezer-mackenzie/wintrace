from collections.abc import Iterator
from typing import Any, TYPE_CHECKING

import wintrace

if TYPE_CHECKING:
    record_events: Iterator[wintrace.EventRecord] = wintrace.iter_evtx("events.evtx")
    dict_events: Iterator[dict[str, Any]] = wintrace.iter_evtx(
        "events.evtx",
        format="dict",
    )
    xml_events: Iterator[str] = wintrace.iter_evtx("events.evtx", format="xml")

    live_records: Iterator[wintrace.EventRecord] = wintrace.iter_events()
    live_dicts: Iterator[dict[str, Any]] = wintrace.iter_events(format="dict")
    live_json: Iterator[str] = wintrace.iter_events(format="json")

    with wintrace.EvtxFile("events.evtx") as event_file:
        file_dicts: Iterator[dict[str, Any]] = event_file.iter(format="dict")

    live_log = wintrace.LiveEventLog()
    log_records: Iterator[wintrace.EventRecord] = live_log.iter()
    log_xml: Iterator[str] = live_log.iter(format="xml")
