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
"""The recipes file, userland/capsule_market/linux-catalogue.json: a
versioned object, {"format": FORMAT, "apps": [recipe, ...]}. A file of any
other format, shape or encoding is refused whole, never read in part."""

import json
from pathlib import Path

from .recipes import checked

FORMAT = "nonos.linux-catalogue.v1"
MAX_BYTES = 1 << 20


def load(path: Path, root: Path) -> list:
    """Every recipe, checked, each package once, or ValueError."""
    raw = path.read_bytes()
    if len(raw) > MAX_BYTES:
        raise ValueError(f"{path}: larger than {MAX_BYTES} bytes")
    try:
        doc = json.loads(raw.decode("utf-8"))
    except (UnicodeDecodeError, json.JSONDecodeError) as why:
        raise ValueError(f"{path}: not JSON: {why}") from None
    if not isinstance(doc, dict) or doc.get("format") != FORMAT:
        raise ValueError(f"{path}: not a {FORMAT} file")
    if set(doc) != {"format", "apps"} or not isinstance(doc["apps"], list):
        raise ValueError(f"{path}: holds keys other than format and apps")
    items = [checked(i, root) for i in doc["apps"]]
    names = [i["package"] for i in items]
    if len(set(names)) != len(names):
        raise ValueError("a package has two recipes")
    return items
