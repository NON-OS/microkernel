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
"""The manifest of public fingerprints: one line per key the build uses, its
algorithm, where its public half is and the SHA-256 of that public half. It holds
nothing secret and is committed beside the public keys."""

import hashlib
import json
import os

MANIFEST = "nonos-data/trust/keys/key-manifest.json"


def fingerprint(path):
    with open(path, "rb") as f:
        return hashlib.sha256(f.read()).hexdigest()


def write(root, keys):
    rows = []
    for k in keys:
        pub = os.path.join(root, k.public)
        if not os.path.exists(pub):
            raise SystemExit(f"{k.public} is missing; run make first")
        rows.append({"key": k.name, "kind": k.kind, "public": k.public, "sha256": fingerprint(pub)})
    with open(os.path.join(root, MANIFEST), "w") as f:
        json.dump({"version": 1, "keys": rows}, f, indent=1)
        f.write("\n")
    return len(rows)
