# NONOS Operating System
# Copyright (C) 2026 NONOS Contributors
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Affero General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
# GNU Affero General Public License for more details.
#
# You should have received a copy of the GNU Affero General Public License
# along with this program. If not, see <https://www.gnu.org/licenses/>.

"""An `unplug:ID` step pulls device ID out through QMP's device_del and fails
the run when QEMU refuses; the USB audio lane's device has an id to pull."""

import sys
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "userland/capsule_install/e2e"))

from nonos_drive import step  # noqa: E402
from nonos_qemu.lane_devices import PLAIN  # noqa: E402


class Machine:
    """Holds one device, lane-usb-audio, and refuses any other id."""

    def __init__(self):
        self.calls = []

    def cmd(self, name, **args):
        self.calls.append((name, args))
        if args.get("id") == "lane-usb-audio":
            return {"return": {}}
        desc = f"Device '{args.get('id')}' not found"
        return {"error": {"class": "DeviceNotFound", "desc": desc}}


class Unplug(unittest.TestCase):
    def test_a_device_the_machine_holds_is_pulled(self):
        q = Machine()
        self.assertTrue(step.run(q, "unplug:lane-usb-audio", Path("."), None))
        self.assertEqual(q.calls, [("device_del", {"id": "lane-usb-audio"})])

    def test_an_id_the_machine_does_not_hold_fails_the_run(self):
        self.assertFalse(step.run(Machine(), "unplug:lane-nothing", Path("."), None))

    def test_the_usb_audio_lane_names_its_device(self):
        self.assertIn("-device usb-audio,id=lane-usb-audio,", PLAIN["usb-audio"])


if __name__ == "__main__":
    unittest.main()
