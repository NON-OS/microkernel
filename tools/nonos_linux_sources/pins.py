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

"""The source recipes and their pins, and whether a tar holds its pin: the
same bundling the seal holds it to (nonos_market_catalogue
source_content.py), so a tar taken here is one the seal takes."""

import json
import shutil
import tempfile
from pathlib import Path

from nonos_market_catalogue import source_content, source_lock
from nonos_market_catalogue.digest import blake3

ROOT = Path(__file__).resolve().parents[2]
CATALOGUE = ROOT / "userland/capsule_market/linux-catalogue.json"
LOCK = ROOT / "userland/capsule_market/linux-catalogue.sources.lock.json"


def recipes(catalogue: Path = CATALOGUE) -> list:
    """Every source recipe of the catalogue, in its order."""
    apps = json.loads(catalogue.read_text())["apps"]
    return [a for a in apps if isinstance(a, dict) and a.get("kind") == "source"]


def pins(lock: Path = LOCK) -> dict:
    """package -> (B, S) from the sources lock; ValueError for a malformed one."""
    out = {}
    for package, row in source_lock.read(lock).items():
        out[package] = (row["B"], int(row["S"]))
    return out


def holds(tar: Path, package: str, pin: tuple) -> bool:
    """Whether `tar`, as <package>.tar, bundles to the pinned bytes."""
    if not tar.is_file() or tar.is_symlink():
        return False
    with tempfile.TemporaryDirectory() as d:
        shutil.copyfile(tar, Path(d) / f"{package}.tar")
        try:
            body = source_content.bundled(Path(d), package)
        except ValueError:
            return False
    return len(body) == pin[1] and blake3(body) == pin[0]
