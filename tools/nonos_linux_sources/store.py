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

"""The content branch: a git branch whose linux-sources/ holds <package>.tar
for each source recipe, built where those builds run and never merged
(its README says so). Read through git alone: its objects are fetched and
a file is written out of them, and nothing in it is run. What it gives is
held to the pin by the caller like anything else."""

import subprocess
from pathlib import Path

REMOTE = "origin"
BRANCH = "content/linux-sources"
DIR = "linux-sources"
SECONDS = 1800


def git(repo: Path, *args, out=None) -> int:
    with open(out, "wb") if out else open("/dev/null", "wb") as sink:
        r = subprocess.run(["git", "-C", str(repo), *args], stdout=sink, stderr=subprocess.PIPE,
                           timeout=SECONDS, check=False)
    return r.returncode


def fetched(repo: Path, remote: str = REMOTE, branch: str = BRANCH) -> bool:
    """The branch's head as FETCH_HEAD, or False when it cannot be had."""
    return git(repo, "fetch", "--quiet", "--no-tags", remote, f"refs/heads/{branch}") == 0


def take(repo: Path, package: str, out: Path) -> bool:
    """Write the branch's <package>.tar to `out`; False when it holds none."""
    part = out.with_name(out.name + ".part")
    if git(repo, "show", f"FETCH_HEAD:{DIR}/{package}.tar", out=part) != 0:
        part.unlink(missing_ok=True)
        return False
    part.replace(out)
    return True
