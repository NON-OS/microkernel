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


def program(files: bytes, name: str, wanted: str = ""):
    """(path, body) of the program, None when there is none, or ValueError
    when the recorded file is not an ELF: the program could not start, and
    a policy naming it would name nothing that runs. `wanted` is a recipe's
    `program`, the path its listing sells; the device records by the bytes
    the policy names (install/market/choose.rs), so the path must be the one
    those bytes lead it to."""
    entries = tarwalk.files(files)
    if wanted:
        return chosen_by_bytes(entries, name, wanted)
    own = f"usr/bin/{name}".encode()
    if any(e.name == own for e in entries):
        chosen = own
    else:
        elves = [e.name for e in entries if e.name.startswith(b"usr/bin/") and e.body.startswith(b"\x7fELF")]
        if not elves:
            return None
        chosen = elves[0]
    body = [e.body for e in entries if e.name == chosen][-1]
    return chosen.decode(errors="replace"), bound(chosen.decode(errors="replace"), body)


def bound(path: str, body: bytes) -> bytes:
    """The program's bytes, or ValueError when they are not an ELF or are
    past what the device binds."""
    if not body.startswith(b"\x7fELF"):
        raise ValueError(f"its {path} is not an ELF")
    if len(body) > MAX_IMAGE:
        raise ValueError(f"its program is past the {MAX_IMAGE >> 20} MiB the device binds")
    return body


def chosen_by_bytes(entries, name: str, wanted: str):
    """choose.rs's rule with a policy: of the usr/bin names whose last copy
    is an ELF of the wanted program's bytes, usr/bin/<name> if one, else the
    first in archive order. It must lead back to `wanted`, or the device
    would record another path than the one the listing sells."""
    want = wanted.encode()
    last = {e.name: e.body for e in entries}
    body = last.get(want)
    if not want.startswith(b"usr/bin/") or body is None:
        raise ValueError(f"its program {wanted} is not in the package")
    bound(wanted, body)
    same = [e.name for e in entries if e.name.startswith(b"usr/bin/") and last[e.name] == body]
    own = f"usr/bin/{name}".encode()
    device = own if own in same else same[0]
    if device != want:
        raise ValueError(f"its program's bytes are also {device.decode(errors='replace')}, which the device would record")
    return wanted, body
