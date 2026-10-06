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
"""Listings for the programs the image ships."""

import json
import sys
from pathlib import Path

from .digest import blake3
from .records import entry, release


# The one line the store shows under a listing's release, kept short enough
# for its detail pane.
SHIPPED = "shipped in the image, pinned by BLAKE3"
MODEL = "model downloaded, pinned by SHA-256"


def note(tail: str) -> str:
    """What the release note says: a Qwen tier's model is downloaded."""
    return MODEL if tail.startswith("qwen-") else SHIPPED


def guest_entries(listing: Path, key: str, when_ms: int) -> list:
    """Programs the image ships, listed so a person can choose one.

    The listing pins the program already in the store by its BLAKE3, and
    the personality installs it from the image. A Qwen tier is that program
    and a model that is not in the image: installing it downloads the
    model, which the kernel keeps only if its SHA-256 is the signed pin, so
    its note says so rather than that it shipped. A program this build did
    not make is left out, not listed unready.
    """
    out = []
    for item in json.loads(listing.read_text()):
        program = Path(item["program"])
        if not program.exists():
            print(f"guest: {item['tail']}: {program} not built, not listed", file=sys.stderr)
            continue
        body = program.read_bytes()
        digest = blake3(body)
        # Tiers share a program; each still needs an id of its own.
        capsule_id = blake3(body + b"\0" + item["tail"].encode())
        rel = release(f"{item['tail']}@1", digest, digest, f"store:/bin/{program.name}",
                      ["x86_64-linux"], [], note(item["tail"]), "nonos.operator.linux", when_ms)
        out.append(entry(f"linux.{item['tail']}", capsule_id, item["name"], "NONOS", key,
                         item["text"], [rel]))
    return out
