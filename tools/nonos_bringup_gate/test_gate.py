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
"""The seal gate on a recorded log and on census lines as the kernel writes
them (src/hardware/boot_census/rule.rs; the same strings hwreport_proofs
reads back). Run with python3 -m unittest
discover -s tools/nonos_bringup_gate -t tools."""

import os
import unittest

from nonos_bringup_gate.gate import LAPTOP, boot_faults, run_faults, trace_faults
from nonos_bringup_gate.census import read

HERE = os.path.dirname(os.path.abspath(__file__))
# A recorded q35 boot of 0.9.3, from before the census existed.
RECORDED = os.path.join(HERE, "..", "..", "docs", "release-evidence", "0.9.3", "wallet-no-tpm-kvm", "serial.log")

GREEN = """\
[CENSUS] taken 4120 ms after init's spawns
[CENSUS] battery up expected=1 up=1
[CENSUS] ac up expected=1 up=1
[CENSUS] lid up expected=1 up=1
[CENSUS] thermal held expected=2 up=0 why=zones declared; their temperatures are the ACPI capsule's own
[CENSUS] keyboard up expected=1 up=1
[CENSUS] pointer up expected=1 up=1
[CENSUS] display up expected=1 up=1
[CENSUS] storage up expected=1 up=1
[CENSUS] network up expected=1 up=1
[CENSUS] usb up expected=1 up=1
[CENSUS] audio up expected=1 up=1
[CENSUS] summary up=10 absent=0 held=1 missing=0 verdict=green
"""

RED = (GREEN.replace("[CENSUS] battery up expected=1 up=1",
                     "[CENSUS] battery missing expected=1 up=0 why=present, its charge not read")
       .replace("up=10 absent=0 held=1 missing=0 verdict=green", "up=9 absent=0 held=1 missing=1 verdict=red"))

BEFORE = """\
[ACPI] EC data=0x62 command=0x66 queries=12
[ACPI] _REG failed in 0 scopes, space 3
[ACPI] _INI ran 9, failed 0; absent 2; _STA failed 1
[ACPI] EC turned off: three waits in a row timed out; its space is refused, its GPE quieted
[ACPI] entered ACPI mode
"""

AFTER = """\
[ACPI] EC data=0x62 command=0x66 queries=12
[ACPI] entered ACPI mode
[ACPI] _REG failed in 0 scopes, space 3
[ACPI] _INI ran 9, failed 0; absent 2; _STA failed 0
[BRINGUP] t=6180 dev=BATTERY ev=published
"""


def boot(i, text):
    # Each boot's log differs in its timing line, as real ones do.
    return (f"boot-{i:02}.log", text.replace("taken 4120", f"taken {4000 + i}"))


class Census(unittest.TestCase):
    def test_a_recorded_boot_from_before_the_census_fails(self):
        with open(RECORDED, encoding="utf-8", errors="replace") as fh:
            text = fh.read()
        faults = boot_faults(read("recorded", text))
        self.assertEqual(faults, ["recorded: no [CENSUS] summary line"])

    def test_a_green_laptop_boot_passes(self):
        self.assertEqual(boot_faults(read("b", GREEN), LAPTOP), [])

    def test_a_missing_battery_fails_with_its_reason(self):
        faults = boot_faults(read("b", RED), LAPTOP)
        self.assertIn("b: battery missing: present, its charge not read", faults)
        self.assertIn("b: battery is missing, must be up", faults)

    def test_the_last_census_said_counts(self):
        self.assertEqual(boot_faults(read("b", RED + GREEN), LAPTOP), [])
        self.assertNotEqual(boot_faults(read("b", GREEN + RED), LAPTOP), [])

    def test_rows_that_do_not_add_up_fail(self):
        bad = GREEN.replace("up=10 absent=0", "up=11 absent=0")
        self.assertIn("b: the census rows do not add up to its summary", boot_faults(read("b", bad)))

    def test_a_fatal_line_fails_a_green_census(self):
        self.assertTrue(boot_faults(read("b", GREEN + "[PANIC] at x\n")))

    def test_serial_noise_before_the_tag_is_read_through(self):
        self.assertEqual(boot_faults(read("b", GREEN.replace("[CENSUS]", "\x1b[0m[CENSUS]")), LAPTOP), [])


class Run(unittest.TestCase):
    def test_twenty_green_boots_pass(self):
        self.assertEqual(run_faults([boot(i, GREEN) for i in range(20)], 20, LAPTOP), [])

    def test_one_red_boot_in_twenty_fails_the_run(self):
        logs = [boot(i, GREEN) for i in range(20)]
        logs[13] = boot(13, RED)
        faults = run_faults(logs, 20, LAPTOP)
        self.assertTrue(any(f.startswith("boot-13.log: battery missing") for f in faults))

    def test_nineteen_boots_are_not_twenty(self):
        self.assertIn("19 boot logs, the gate needs 20", run_faults([boot(i, GREEN) for i in range(19)], 20))

    def test_the_same_log_twice_counts_once(self):
        logs = [boot(i, GREEN) for i in range(19)] + [("again.log", boot(3, GREEN)[1])]
        self.assertTrue(any("the same log as boot-03.log" in f for f in run_faults(logs, 20)))


class Trace(unittest.TestCase):
    def test_the_old_order_then_the_new_shows_the_race_gone(self):
        self.assertEqual(trace_faults(BEFORE, AFTER), [])

    def test_an_after_log_with_the_old_order_fails(self):
        faults = trace_faults(BEFORE, BEFORE)
        self.assertIn("after: ACPI mode is not entered before the first _REG", faults)
        self.assertIn("after: the EC was turned off 1 time(s)", faults)
        self.assertIn("after: no [BRINGUP] line with the battery published", faults)


if __name__ == "__main__":
    unittest.main()
