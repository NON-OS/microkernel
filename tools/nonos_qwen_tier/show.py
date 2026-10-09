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
"""`show`: a catalogue as text, its signature checked when a key is given."""

import sys

from .ed25519 import verify
from .wire_read import decode


def show(path, pubkey):
    with open(path, "rb") as f:
        blob = f.read()
    if not blob:
        sys.exit(f"{path}: empty; it was built without the operator seed")
    serial, base, tiers, body, sig = decode(blob)
    print(f"serial {serial}, NONOS mirror {base or '(none)'}, {len(tiers)} tiers")
    for word, memory, files in tiers:
        print(f"{word}: needs {memory} bytes of memory")
        for name, length, sha, urls in files:
            print(f"  {name} {length} {sha}")
            print("".join(f"    {u}\n" for u in urls), end="")
    if pubkey:
        with open(pubkey, "rb") as k:
            if not verify(k.read(), body, sig):
                sys.exit(f"{path}: the signature does not verify under {pubkey}")
        print("signature verified")
