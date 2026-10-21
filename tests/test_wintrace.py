import json
import os
import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path
from typing import Any, cast

import wintrace


EVENT_XML = """\
<Event>
  <System>
    <Provider Name="Example-Provider" />
    <EventID>4624</EventID>
    <EventRecordID>100</EventRecordID>
    <Level>4</Level>
    <TimeCreated SystemTime="2026-01-02T03:04:05Z" />
    <Channel>Security</Channel>
    <Computer>host.example.test</Computer>
  </System>
  <EventData>
    <Data Name="TargetUserName">alice</Data>
    <Data Name="LogonType">3</Data>
  </EventData>
</Event>
"""


class TestXmlUtilities(unittest.TestCase):
    def test_xml_to_dict_preserves_event_structure(self):
        result = wintrace.xml_to_dict(EVENT_XML)
        self.assertEqual(result["Event"]["System"]["EventID"], "4624")
        self.assertEqual(
            result["Event"]["System"]["Provider"]["@Name"],
            "Example-Provider",
        )

    def test_xml_to_json_is_valid_and_keeps_repeated_fields(self):
        result = json.loads(wintrace.xml_to_json(EVENT_XML))
        self.assertEqual(
            result["Event"]["EventData"]["Data"][1]["#text"],
            "3",
        )

    def test_malformed_xml_raises_value_error(self):
        with self.assertRaises(ValueError):
            wintrace.xml_to_dict("<Event>")


class TestOfflineApi(unittest.TestCase):
    def test_unknown_format_is_rejected(self):
        with self.assertRaises(ValueError):
            wintrace.read_evtx("missing.evtx", format=cast(Any, "yaml"))

    def test_missing_file_is_reported(self):
        with self.assertRaises(FileNotFoundError):
            wintrace.iter_evtx("not-a-real-wintrace-file.evtx")

    def test_pathlike_is_accepted(self):
        with self.assertRaises(FileNotFoundError):
            wintrace.iter_evtx(Path("not-a-real-wintrace-file.evtx"))

    def test_invalid_file_is_rejected(self):
        with tempfile.NamedTemporaryFile(delete=False) as file:
            file.write(b"not an EVTX file")
            path = file.name
        try:
            with self.assertRaises(ValueError):
                wintrace.EvtxFile(path)
        finally:
            os.unlink(path)


class TestPackageSurface(unittest.TestCase):
    def test_version_and_exports(self):
        self.assertEqual(wintrace.__version__, "0.1.0")
        for name in wintrace.__all__:
            self.assertTrue(hasattr(wintrace, name), name)

    def test_live_queries_are_platform_gated(self):
        if os.name != "nt":
            with self.assertRaises(NotImplementedError):
                wintrace.iter_events()


@unittest.skipUnless(sys.platform == "win32", "Windows Event Log integration test")
class TestWindowsIntegration(unittest.TestCase):
    def test_live_formats_and_offline_export(self):
        events = wintrace.query_events(channel="System", limit=1)
        if not events:
            self.skipTest("The System channel has no readable events")

        event = events[0]
        self.assertIsInstance(event, wintrace.EventRecord)
        self.assertEqual(event["System"], event.system)
        self.assertEqual(event.get("missing"), None)
        self.assertEqual(event.get("missing", "fallback"), "fallback")
        self.assertIsInstance(
            wintrace.query_events(channel="System", limit=1, format="dict")[0],
            dict,
        )
        self.assertIsInstance(
            wintrace.query_events(channel="System", limit=1, format="xml")[0],
            str,
        )
        self.assertIsInstance(
            json.loads(
                wintrace.query_events(
                    channel="System", limit=1, format="json"
                )[0]
            ),
            dict,
        )

        wevtutil = shutil.which("wevtutil.exe")
        if not wevtutil or event.record_id is None:
            self.skipTest("Exporting a single EVTX record is unavailable")
        with tempfile.TemporaryDirectory() as directory:
            path = os.path.join(directory, "single-event.evtx")
            query = f"*[System[EventRecordID={event.record_id}]]"
            subprocess.run(
                [
                    wevtutil,
                    "epl",
                    "System",
                    path,
                    f"/q:{query}",
                    "/ow:true",
                ],
                check=True,
                capture_output=True,
                text=True,
            )
            offline = wintrace.read_evtx(path, limit=1)
        self.assertTrue(offline)
        self.assertEqual(offline[0].record_id, event.record_id)


if __name__ == "__main__":
    unittest.main()
