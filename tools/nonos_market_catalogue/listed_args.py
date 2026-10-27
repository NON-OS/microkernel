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
"""A recipe's `args`: the long options its program starts with, before the
person's own, in nonos_linux_policy::args's block, which rides after the
policy in the release's guest_policy and so under its signature. Each is
one argv word as written: `--name` or `--name=value`, a name of lowercase
letters, digits and hyphens, a value of printable ASCII with no space, no
option twice, 1 to 16 words, the block at most 1024 bytes. A person's own
option of the same name replaces the listed one on the device."""

import re

MAGIC = b"NLXARG01"
MAX_LEN = 1024
MAX_WORDS = 16
WORD = re.compile(rb"--[a-z0-9][a-z0-9-]*(=[\x21-\x7e]+)?")


def block(words: list) -> bytes:
    """The canonical block for `words`, or ValueError naming the rule."""
    if not isinstance(words, list) or not all(isinstance(w, str) for w in words):
        raise ValueError("args is a list of strings")
    if not 1 <= len(words) <= MAX_WORDS:
        raise ValueError(f"{len(words)} args: a recipe lists 1 to {MAX_WORDS}")
    raw = [w.encode() for w in words]
    for w in raw:
        if not WORD.fullmatch(w) or len(w) > 255:
            raise ValueError(f"{w!r} is not --name or --name=value")
    names = [w.split(b"=", 1)[0] for w in raw]
    if len(set(names)) != len(names):
        raise ValueError("an option is listed twice")
    out = MAGIC + bytes([len(raw)]) + b"".join(bytes([len(w)]) + w for w in raw)
    if len(out) > MAX_LEN:
        raise ValueError(f"the args take {len(out)} bytes, past {MAX_LEN}")
    return out


def listed(item: dict) -> bytes:
    """The recipe's block, or b"" when it lists none."""
    return block(item["args"]) if "args" in item else b""
