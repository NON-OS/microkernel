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
"""SERVE_ONE's field, as nonos_linux_policy's serve.rs encodes it: in a
version 2 policy, at byte 217, the port little-endian and then the name of
the one capsule the kernel lets reach it, zero padded to 32 bytes. A recipe
writes it as `serve`, `name:port`. test_serve.py holds it to the crate."""

import re

VERSION_SERVE, AT_SERVE, NAME_MAX = 2, 217, 32
# ServeOne::new: 1 to 32 bytes of [a-z0-9._-], from a letter.
NAME = re.compile(r"[a-z][a-z0-9._-]{0,31}")
# A port from 1 to 65535, in one spelling: no sign and no leading zero.
PORT = re.compile(r"[1-9][0-9]{0,4}")


def serve_of(text) -> tuple:
    """`name:port` to (port, name), or ValueError."""
    name, _, port = text.rpartition(":") if isinstance(text, str) else ("", "", "")
    if not NAME.fullmatch(name):
        raise ValueError(f"serve {text!r}: the name is not 1 to 32 of [a-z0-9._-] from a letter")
    if not PORT.fullmatch(port) or int(port) > 0xFFFF:
        raise ValueError(f"serve {text!r}: the port is not 1 to 65535")
    return int(port), name


def field(serve: tuple) -> bytes:
    """The 34 bytes from 217, or ValueError for a port or name ServeOne refuses."""
    port, name = serve
    if not 0 < port <= 0xFFFF or not NAME.fullmatch(name):
        raise ValueError("SERVE_ONE's port or name is one the policy crate refuses")
    return port.to_bytes(2, "little") + name.encode().ljust(NAME_MAX, b"\0")


def served(mask: int, text) -> tuple:
    """A recipe's mask with SERVE_ONE, and its (port, name), or ValueError."""
    from .policy import MASK  # policy.py imports this file
    if mask & MASK["WAYLAND"]:
        raise ValueError(f"serve {text!r}: SERVE_ONE with WAYLAND is not market ready")
    return mask | MASK["SERVE_ONE"], serve_of(text)
