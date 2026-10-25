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
"""nonos-cap-audit: which declared capabilities something consults."""
"""nonos-cap-audit on fixtures: a capability only built into a capsule's mask
is not consulted, and one a gate asks about, by `.bit()` or not, is. A
constant made from a bit counts only where something reads it, and a `use`
re-exporting it reads nothing. An entry with its doc as an attribute on the
same line is declared like any other."""

import tempfile
from pathlib import Path

from nonos_cap_audit_consulted import consulted
from nonos_cap_audit_read import declared

FIXTURES = {
    "src/capabilities/types/defs.rs": "    #[doc = \"Enforces nothing.\"] IO = 1 << 1,\n"
    "    /// Debug.\n    Debug = 1 << 8,\n    Defined = 1 << 9,\n    Checked = 1 << 10,\n",
    "src/caps/types.rs": "pub const CAP_DEFINED: u64 = Capability::Defined.bit();\n"
    "pub const CAP_CHECKED: u64 = Capability::Checked.bit();\n",
    "src/caps/mod.rs": "pub use types::{\n    CAP_CHECKED, CAP_DEFINED,\n};\n",
    "src/gate.rs": "fn gate(c: u64) -> bool {\n    c & CAP_CHECKED != 0\n}\n",
    "src/userspace/probe.rs": "fn probe() -> u64 {\n    base | Capability::IO.bit()\n}\n",
    "src/spawn/gate.rs": "fn spawn(c: u32) -> bool {\n    has(c, Capability::Debug.bit())\n}\n",
}


def self_test() -> int:
    with tempfile.TemporaryDirectory() as d:
        root = Path(d)
        for rel, text in FIXTURES.items():
            (root / rel).parent.mkdir(parents=True, exist_ok=True)
            (root / rel).write_text(text)
        listed = declared(root)
        got = consulted(root, listed)
    if listed != {"IO", "Debug", "Defined", "Checked"}:
        print(f"cap-audit: self-test failed, declared {sorted(listed)}")
        return 1
    want = {"Debug", "Checked"}
    if got != want:
        print(f"cap-audit: self-test failed, consulted {sorted(got)}, wanted {sorted(want)}")
        return 1
    print("cap-audit: self-test passed: every entry is declared, and a mask and "
          "a defined constant are no check, a gate is")
    return 0
