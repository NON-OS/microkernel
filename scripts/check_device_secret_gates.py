#!/usr/bin/env python3
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
"""Where the kernel opens a DeviceSecret gate.

The cap is signed to nonos.prove alone (check_device_secret_cap.py), and that
is only worth something while every gate it opens is one somebody reviewed.
This lists them: each file here must still test the cap, so a gate that is
dropped fails, and a test of the cap anywhere else fails until it is added.
"""

from pathlib import Path

CALL = "can_device_secret()"

# file -> what the cap opens there
GATES = {
    "src/syscall/contract/cap_table/mk.rs": "MkDeviceSecret, MkBootSlots, MkEnroll",
    "src/syscall/contract/cap_table/mk_trust.rs": "MkSealRead",
    "src/syscall/microkernel/device_proof/gate.rs": "device_secret_caller(): the secret, and MkSealRead's witness",
}


def gate_problems(root):
    found = {str(p.relative_to(root)) for p in (root / "src").rglob("*.rs") if CALL in p.read_text()}
    lost = [f"{f} no longer gates {what} on DeviceSecret" for f, what in GATES.items() if f not in found]
    new = [f"{f} tests DeviceSecret and is not a reviewed gate" for f in sorted(found - set(GATES))]
    return lost + new


if __name__ == "__main__":
    import sys

    problems = gate_problems(Path("."))
    for p in problems:
        print(f"device-secret-gates: {p}", file=sys.stderr)
    print(f"device-secret-gates: {len(GATES)} gates, {len(problems)} problems")
    sys.exit(1 if problems else 0)
