"""Query the Windows Event Log service."""

from __future__ import annotations

from collections.abc import Iterator
from typing import Any, Literal, overload

from ._types import OutputFormat
from ._wintrace import EventRecord, LiveEventIterator, LiveEventLog
from ._wintrace import (
    iter_events as _iter_events,
)
from ._wintrace import (
    query_events as _query_events,
)


@overload
def query_events(
    channel: str = "System",
    query: str = "*",
    limit: int | None = None,
    reverse: bool = True,
    format: Literal["record"] = "record",
) -> list[EventRecord]: ...


@overload
def query_events(
    channel: str,
    query: str,
    limit: int | None,
    reverse: bool,
    format: Literal["dict"],
) -> list[dict[str, Any]]: ...


@overload
def query_events(
    channel: str,
    query: str,
    limit: int | None,
    reverse: bool,
    format: Literal["xml", "json"],
) -> list[str]: ...


@overload
def query_events(
    channel: str = "System",
    query: str = "*",
    limit: int | None = None,
    reverse: bool = True,
    *,
    format: Literal["dict"],
) -> list[dict[str, Any]]: ...


@overload
def query_events(
    channel: str = "System",
    query: str = "*",
    limit: int | None = None,
    reverse: bool = True,
    *,
    format: Literal["xml", "json"],
) -> list[str]: ...


def query_events(
    channel: str = "System",
    query: str = "*",
    limit: int | None = None,
    reverse: bool = True,
    format: OutputFormat = "record",
) -> list[Any]:
    """Read matching events from a local Windows Event Log channel."""
    return _query_events(
        channel=channel,
        query=query,
        limit=limit,
        reverse=reverse,
        format=format,
    )


@overload
def iter_events(
    channel: str = "System",
    query: str = "*",
    reverse: bool = True,
    format: Literal["record"] = "record",
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
    channel: str = "System",
    query: str = "*",
    reverse: bool = True,
    *,
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
    channel: str = "System",
    query: str = "*",
    reverse: bool = True,
    *,
    format: Literal["xml", "json"],
) -> Iterator[str]: ...


def iter_events(
    channel: str = "System",
    query: str = "*",
    reverse: bool = True,
    format: OutputFormat = "record",
) -> Iterator[Any]:
    """Return an iterator over events matching a Windows XPath query."""
    return _iter_events(
        channel=channel,
        query=query,
        reverse=reverse,
        format=format,
    )


__all__ = [
    "LiveEventIterator",
    "LiveEventLog",
    "iter_events",
    "query_events",
]