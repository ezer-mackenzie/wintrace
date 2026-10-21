"""Conversion helpers for Windows Event XML."""

from ._wintrace import xml_to_dict, xml_to_json

__all__ = ["xml_to_dict", "xml_to_json"]
