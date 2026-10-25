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
"""The constants a block driver's protocol states: its protocol and wire files,
and the shared block wire's or the SCSI server's copy of it, when it takes those."""

import re

WIRE = "userland/nonos_blk_wire/src"
SERVE = "userland/nonos_scsi_serve/src/wire"


def protocol(root, crate, consts):
    """The constants `crate`'s protocol files state. A capsule on the shared block
    wire takes them from nonos_blk_wire by path, so that crate's count as its own,
    and a constant the capsule defines itself wins over the shared one."""
    own = sorted((crate / "src/protocol").glob("*.rs")) + sorted((crate / "src/wire").glob("*.rs"))
    shared = any(re.search(r"\bnonos_blk_wire::", f.read_text()) for f in own)
    # A SCSI adapter's driver serves the wire through nonos_scsi_serve's copy.
    toml = crate / "Cargo.toml"
    served = toml.is_file() and "nonos_scsi_serve" in toml.read_text()
    serve = sorted((root / SERVE).glob("*.rs")) if served else []
    return consts((sorted((root / WIRE).glob("*.rs")) if shared else []) + serve + own)
