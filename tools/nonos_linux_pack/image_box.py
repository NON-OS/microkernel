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

"""The box an image's listing asks for, beside its entry: the services by
name (policy.py MASK) and the program the policy will name, found in the
image's members with its BLAKE3, which the project's signing step puts in the
release's guest policy. Nothing here signs or writes a policy."""

from nonos_market_catalogue.tarwalk import walk

from .image_hash import digest
from .policy import MASK, mask_of


def program(members: list, path: str) -> bytes:
    """The bytes of the last regular file at `path` in any member, as the
    device places them (a later member's copy lands over an earlier's);
    ValueError when no member has one, or it is not an ELF."""
    want = path.lstrip("/").encode()
    found = None
    for m in members:
        for e in walk(m.tar):
            if e.name == want and e.kind == "file":
                found = e.body
    if found is None:
        raise ValueError(f"no member places {path}")
    if not found.startswith(b"\x7fELF"):
        raise ValueError(f"{path} is not an ELF")
    return found


def bound_at(members: list, body: bytes):
    """The path the device binds to for a program of `body`'s bytes
    (capsule_linux install/image/bound.rs and market/choose.rs, anywhere):
    in the first member, in image order, that carries them as the last copy
    of a name, the first such name in its archive order."""
    want = digest(body)
    for m in members:
        files = [e for e in walk(m.tar) if e.kind == "file"]
        last = {e.name: e.body for e in files}
        names = [n for n in dict.fromkeys(e.name for e in files)
                 if last[n].startswith(b"\x7fELF") and digest(last[n]) == want]
        if names:
            return names[0].decode(errors="replace")
    return None


def box(members: list, mask: str, path: str) -> dict:
    """The box: its services, their bits, the program and its BLAKE3.
    FILES_USER grants the person's Documents and Downloads, which the
    user-files broker serves."""
    bits = mask_of(mask)
    names = sorted((n for n, b in MASK.items() if bits & b), key=lambda n: MASK[n])
    body = program(members, path)
    if bound_at(members, body) != path.lstrip("/"):
        raise ValueError(f"the device would bind {bound_at(members, body)}, not {path}: the same bytes come first there")
    return {"mask": names, "mask_bits": f"{bits:#x}", "program": "/" + path.lstrip("/"),
            "program_blake3": digest(body).hex()}
