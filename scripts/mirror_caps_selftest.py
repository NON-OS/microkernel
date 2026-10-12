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
"""check_mirror_caps's self-test: decoys it must pass and drift it must report."""

import tempfile
from pathlib import Path

from mirror_caps_bits import DEFS
from mirror_caps_readme import stale_readmes
from mirror_caps_wider import wider


def self_test():
    with tempfile.TemporaryDirectory() as tmp:
        root = Path(tmp)
        (root / "src/capabilities/types").mkdir(parents=True)
        (root / DEFS).write_text("    IPC = 1 << 3,\n    Memory = 1 << 4,\n    Debug = 1 << 8,\n    Crypto = 1 << 5,\n")
        (root / "userland/capsule_decoy").mkdir(parents=True)
        (root / "userland/capsule_decoy/Capsule.mk").write_text(
            "CAPSULE_SERVICE_ENDPOINT := service:1:decoy\nCAPSULE_REQUIRED_CAPS := 0x18\n"
            "CAPSULE_OPTIONAL_CAPS := 0x100\n")
        spawn = root / "src/decoy/spawn.rs"
        spawn.parent.mkdir(parents=True)
        body = 'const SERVICE_NAME: &str = "decoy";\n    requested_caps: Capability::IPC.bit() | {},\n'
        spawn.write_text(body.format("crate::capabilities::serial_debug_cap()"))
        assert not wider(root)[0], "a grant within the manifest was reported"
        spawn.write_text(body.format("Capability::Crypto.bit()"))
        assert wider(root)[0], "a grant past the manifest was missed"
        spawn.write_text('const SERVICE_NAME: &str = "decoy";\nconst CAPS: u64 = 0x19;\n    requested_caps: CAPS,\n')
        assert wider(root)[0], "a constant past the manifest was missed"
        readme = root / "userland/capsule_decoy/README.md"
        readme.write_text("Mask: `CAPSULE_REQUIRED_CAPS = 0x18`.\n")
        assert not stale_readmes(root), "a README quoting its manifest was reported"
        readme.write_text("Mask: `CAPSULE_REQUIRED_CAPS = 0x19`.\n")
        assert stale_readmes(root), "a README quoting an old mask was missed"
    print("mirror caps: self-test passed, the decoys were reported")
