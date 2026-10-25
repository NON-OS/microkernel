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
"""A job placed in a development image's copy of the tree, never the checkout.

The headless Terminal (feature agentproof-headless) reads the job at
/linux/usr/share/agentproof/job; the agent's files sit beside it, in the
image's Linux tree, packed and verified with the rest of the store. Only
tools/nonos-dev-image calls this, after it has synced the copy, so the next
build without a job copies the checkout's capsules.json and store.json back.
"""

import json
from pathlib import Path

DIR = "agentproof-job"
LINUX_AT = "/linux/usr/share/agentproof/"
MAX_FILES = 16
MAX_BYTES = 1 << 20


def files(job_dir: Path) -> list:
    """The job and the files beside it: flat, named plainly, small."""
    found = sorted(p for p in job_dir.iterdir() if p.is_file())
    names = [p.name for p in found]
    if "job" not in names:
        raise SystemExit(f"agentproof: {job_dir} has no `job` file")
    if len(found) > MAX_FILES or any(p.stat().st_size > MAX_BYTES for p in found):
        raise SystemExit(f"agentproof: at most {MAX_FILES} files of at most 1 MiB each")
    for n in names:
        if not all(c.isalnum() or c in "._-" for c in n) or n.startswith("."):
            raise SystemExit(f"agentproof: {n!r}: a file is named with letters, digits, . _ -")
    return found


def place(tree: Path, job_dir: Path):
    """Copy the job's files into the copy, list them in its store, and build
    its Terminal headless."""
    out = tree / DIR
    out.mkdir(exist_ok=True)
    for old in out.iterdir():
        old.unlink()
    entries = []
    for f in files(job_dir):
        (out / f.name).write_bytes(f.read_bytes())
        entries.append({"path": LINUX_AT + f.name, "file": f"{DIR}/{f.name}"})
    store = tree / "tools/nix/store.json"
    groups = json.loads(store.read_text())
    groups["linux"] = [e for e in groups["linux"] if not e["path"].startswith(LINUX_AT)] + entries
    store.write_text(json.dumps(groups, indent=1) + "\n")
    caps = tree / "tools/nix/capsules.json"
    catalogue = json.loads(caps.read_text())
    for e in catalogue:
        if e["slug"] == "terminal":
            e["cargo_features"] = "agentproof-headless"
    caps.write_text(json.dumps(catalogue, indent=1) + "\n")
