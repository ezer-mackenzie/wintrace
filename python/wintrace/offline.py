"""Read Windows Event Log archive files."""

from __future__ import annotations

from os import PathLike
from typing import Any, Literal, overload

from ._types import OutputFormat
from ._wintrace import (
    EventRecord,
    EvtxRecordIterator,
)
from ._wintrace import (
    iter_evtx as _iter_evtx,
)
from ._wintrace import (
    read_evtx as _read_evtx,
)


@overload
def read_evtx(
    path: str | PathLike[str],
    limit: int | None = None,
    format: Literal["record"] = "record",
) -> list[EventRecord]: ...


@overload
def read_evtx(
    path: str | PathLike[str],
    limit: int | None,
    format: Literal["dict"],
) -> list[dict[str, Any]]: ...


@overload
def read_evtx(
    path: str | PathLike[str],
    limit: int | None,
    format: Literal["xml", "json"],
) -> list[str]: ...


@overload
def read_evtx(
    path: str | PathLike[str],
    limit: int | None = None,
    *,
    format: Literal["dict"],
) -> list[dict[str, Any]]: ...


@overload
def read_evtx(
    path: str | PathLike[str],
    limit: int | None = None,
    *,
    format: Literal["xml", "json"],
) -> list[str]: ...


def read_evtx(
    path: str | PathLike[str],
    limit: int | None = None,
    format: OutputFormat = "record",
) -> list[Any]:
    """Read records from an EVTX file, optionally limiting the result count."""
    return _read_evtx(path, limit=limit, format=format)


def iter_evtx(
    path: str | PathLike[str],
    format: OutputFormat = "record",
) -> EvtxRecordIterator:
    """Iterate through an EVTX file without returning a list of all its records."""
    return _iter_evtx(path, format=format)
