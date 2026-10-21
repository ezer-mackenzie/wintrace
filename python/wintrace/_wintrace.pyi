from collections.abc import Iterator
from os import PathLike
from types import TracebackType
from typing import Any, Literal, overload

__version__: str

class EventRecord:
    xml: str
    event_id: None | int
    record_id: None | int
    channel: None | str
    provider: None | str
    level: None | int
    time_created: None | str
    computer: None | str
    @property
    def system(self) -> None | dict[str, Any]: ...
    @property
    def event_data(self) -> Any: ...
    def to_dict(self) -> dict[str, Any]: ...
    def to_json(self) -> str: ...
    def get(self, key: str, default: Any = ...) -> Any: ...
    def __getitem__(self, key: str) -> Any: ...

class EvtxRecordIterator(Iterator[Any]):
    def __next__(self) -> Any: ...

class EvtxFile:
    def __init__(self, path: str | PathLike[str]) -> None: ...
    def read(self, limit: None | int = ..., format: str = ...) -> list[Any]: ...
    @overload
    def iter(self, format: Literal["record"] = ...) -> Iterator[EventRecord]: ...
    @overload
    def iter(self, format: Literal["dict"]) -> Iterator[dict[str, Any]]: ...
    @overload
    def iter(self, format: Literal["xml", "json"]) -> Iterator[str]: ...
    def __enter__(self) -> "EvtxFile": ...
    def __exit__(
        self,
        exc_type: type[BaseException] | None,
        exc_value: BaseException | None,
        traceback: TracebackType | None,
    ) -> bool: ...

class LiveEventIterator(Iterator[Any]):
    def __next__(self) -> Any: ...

class LiveEventLog:
    def __init__(
        self,
        channel: str = ...,
        query: str = ...,
        reverse: bool = ...,
    ) -> None: ...
    def read(self, limit: None | int = ..., format: str = ...) -> list[Any]: ...
    @overload
    def iter(self, format: Literal["record"] = ...) -> Iterator[EventRecord]: ...
    @overload
    def iter(self, format: Literal["dict"]) -> Iterator[dict[str, Any]]: ...
    @overload
    def iter(self, format: Literal["xml", "json"]) -> Iterator[str]: ...

def read_evtx(
    path: str | PathLike[str],
    limit: None | int = ...,
    format: str = ...,
) -> list[Any]: ...
@overload
def iter_evtx(
    path: str | PathLike[str],
    format: Literal["record"] = ...,
) -> Iterator[EventRecord]: ...

@overload
def iter_evtx(
    path: str | PathLike[str],
    format: Literal["dict"],
) -> Iterator[dict[str, Any]]: ...

@overload
def iter_evtx(
    path: str | PathLike[str],
    format: Literal["xml", "json"],
) -> Iterator[str]: ...

def iter_evtx(
    path: str | PathLike[str],
    format: str = ...,
) -> Iterator[Any]: ...
def query_events(
    channel: str = ...,
    query: str = ...,
    limit: None | int = ...,
    reverse: bool = ...,
    format: str = ...,
) -> list[Any]: ...
@overload
def iter_events(
    channel: str = ...,
    query: str = ...,
    reverse: bool = ...,
    format: Literal["record"] = ...,
) -> Iterator[EventRecord]: ...

@overload
def iter_events(
    channel: str,
    query: str,
    reverse: bool,
    format: Literal["dict"],
) -> Iterator[dict[str, Any]]: ...

@overload
def iter_events(
    channel: str,
    query: str,
    reverse: bool,
    format: Literal["xml", "json"],
) -> Iterator[str]: ...

@overload
def iter_events(
    channel: str = ...,
    query: str = ...,
    reverse: bool = ...,
    *,
    format: Literal["dict"],
) -> Iterator[dict[str, Any]]: ...

@overload
def iter_events(
    channel: str = ...,
    query: str = ...,
    reverse: bool = ...,
    *,
    format: Literal["xml", "json"],
) -> Iterator[str]: ...

def iter_events(
    channel: str = ...,
    query: str = ...,
    reverse: bool = ...,
    format: str = ...,
) -> Iterator[Any]: ...
def xml_to_dict(xml: str) -> dict[str, Any]: ...
def xml_to_json(xml: str) -> str: ...
