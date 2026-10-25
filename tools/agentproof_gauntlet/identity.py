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
"""What a Gauntlet result names, so it cannot be detached from what ran: the
job's files by hash, and the commits of the image's copy and of the tree."""

import hashlib
import subprocess
from pathlib import Path


def job_hash(job: Path) -> str:
    """SHA-256 over each file's name and bytes, in name order: what ran."""
    h = hashlib.sha256()
    for f in sorted(p for p in job.iterdir() if p.is_file()):
        data = f.read_bytes()
        h.update(f"{f.name}\0{len(data)}\0".encode() + data)
    return h.hexdigest()


def commit(path: Path) -> str:
    r = subprocess.run(["git", "rev-parse", "HEAD"], cwd=path, capture_output=True, text=True)
    return r.stdout.strip()
