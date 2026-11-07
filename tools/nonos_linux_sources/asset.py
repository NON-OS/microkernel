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

"""A recipe that lists upstream's release binary, made from upstream by
tools/nonos-source-asset: upstream's checksum file verified under the
pinned release key, the asset held to it and to the recipe, and, for a
Debian-family app, its needs resolved and pinned. Nothing fetched is run."""

import json
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

from .pins import ROOT

TOOL = ROOT / "tools/nonos-source-asset"
SECONDS = 3600
# What the tool holds upstream's release to its pin with: the checksum
# file's signature (gpg, gpgv) and the content's BLAKE3 (b3sum).
TOOLS = ("b3sum", "gpg", "gpgv")


def is_asset(recipe: dict) -> bool:
    return "asset" in recipe


def missing() -> list:
    """The tools making a release's content needs that are not on PATH here."""
    return [t for t in TOOLS if shutil.which(t) is None]


def make(recipe: dict, out_dir: Path) -> bool:
    """<package>.tar written to `out_dir` from upstream; False when the tool
    stopped, its reason on stderr."""
    with tempfile.TemporaryDirectory() as d:
        spec = Path(d) / "recipe.json"
        spec.write_text(json.dumps(recipe))
        r = subprocess.run([sys.executable, str(TOOL), str(spec), "--out", str(out_dir)], cwd=ROOT,
                           timeout=SECONDS, check=False)
    return r.returncode == 0
