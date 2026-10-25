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
"""The program capsule_linux records for an installed package, and so the
one its policy must name (install/record/program.rs): `usr/bin/<name>` if
the package has a regular file there, ELF or not, else its first ELF under
usr/bin, in archive order. The bytes that land are the last copy of that
name, since the installer writes entries in order."""

from . import tarwalk

# install/market/bind.rs reads a bound program through store_read(.., MAX_IMAGE).
MAX_IMAGE = 64 << 20


def program(files: bytes, name: str):
    """(path, body) of the program, None when there is none, or ValueError
    when the recorded file is not an ELF: the program could not start, and
    a policy naming it would name nothing that runs."""
    entries = tarwalk.files(files)
    own = f"usr/bin/{name}".encode()
    if any(e.name == own for e in entries):
        chosen = own
    else:
        elves = [e.name for e in entries if e.name.startswith(b"usr/bin/") and e.body.startswith(b"\x7fELF")]
        if not elves:
            return None
        chosen = elves[0]
    body = [e.body for e in entries if e.name == chosen][-1]
    if not body.startswith(b"\x7fELF"):
        raise ValueError(f"its {chosen.decode(errors='replace')} is not an ELF")
    if len(body) > MAX_IMAGE:
        raise ValueError(f"its program is past the {MAX_IMAGE >> 20} MiB the device binds")
    return chosen.decode(errors="replace"), body
