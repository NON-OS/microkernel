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
"""check_driver_scope.py's self-test: decoy rows and capsules that it must
pass, and drift it must report."""

import tempfile
from pathlib import Path

from check_driver_scope import drift
from driver_scope_rows import BROKER, CAPSULES


def self_test():
    with tempfile.TemporaryDirectory() as tmp:
        root = Path(tmp)
        (root / BROKER).mkdir(parents=True)
        for f in ["scope_rows.rs", "scope_hosts.rs", "scope_ids_nic.rs", "scope_lpss.rs"]:
            (root / BROKER / f).write_text("")
        (root / BROKER / "scope_ids.rs").write_text("pub(crate) const DECOY: &[u16] = &[\n    0x1000, 0x1007,\n];\n")
        (root / BROKER / "scope_table.rs").write_text('    ("driver.decoy0", &[Ids(0x8086, DECOY)]),\n')
        cap = root / (CAPSULES + "decoy/ids.rs")
        cap.parent.mkdir(parents=True)
        case = [("driver.decoy0", "decoy/ids.rs", "IDS")]
        cap.write_text("// 0x9999 in a comment\npub const IDS: &[u16] = &[0x1000, 0x1007];\n")
        assert not drift(root, case), "ids inside the row were reported"
        cap.write_text("pub const IDS: &[u16] = &[0x1000, 0x1008];\n")
        assert drift(root, case), "an id past the row was missed"
        cap.write_text("fn ids(d: u16) -> bool { matches!(d, 0x1000..=0x1008) }\n")
        assert drift(root, [("driver.decoy0", "decoy/ids.rs", "ids")]), "a range past it was missed"
        assert drift(root, [("driver.gone0", "decoy/ids.rs", "IDS")]), "a missing row was missed"
        assert drift(root, [("driver.decoy0", "decoy/ids.rs", "GONE")]), "a missing item was missed"
        cap.write_text("pub const IDS: &[u16] = &[0x1000];\n")
        assert not drift(root, case), "a row range the table names in part was reported"
        (root / BROKER / "scope_ids.rs").write_text(
            "pub(crate) const DECOY: &[u16] = &[\n    0x1000, 0x1007, 0x2000, 0x2000,\n];\n"
        )
        assert drift(root, case), "a row range the table never names was missed"
        # A row whose ids live in scope_ids_nic.rs is read from there.
        (root / BROKER / "scope_ids.rs").write_text("")
        (root / BROKER / "scope_ids_nic.rs").write_text("pub(crate) const DECOY: &[u16] = &[0x1000, 0x1000];\n")
        assert not drift(root, case), "a row in scope_ids_nic.rs was not read"
    print("driver scope: self-test passed, the decoys were reported")
