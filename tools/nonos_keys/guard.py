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
"""Refusals before anything is made: a private path git would track, or a key
that already exists. A key is never overwritten; rotation is its own decision."""

import os
import subprocess


def in_git(root):
    r = subprocess.run(["git", "-C", root, "rev-parse", "--is-inside-work-tree"], capture_output=True, text=True)
    return r.returncode == 0 and r.stdout.strip() == "true"


def tracked_or_unignored(root, path):
    """Why git would keep this private path, or None."""
    if subprocess.run(["git", "-C", root, "ls-files", "--error-unmatch", path], capture_output=True).returncode == 0:
        return "tracked"
    if subprocess.run(["git", "-C", root, "check-ignore", "-q", path]).returncode != 0:
        return "not ignored"
    return None


def check(root, keys):
    problems = []
    if in_git(root):
        for k in keys:
            why = tracked_or_unignored(root, k.private)
            if why:
                problems.append(f"{k.private}: {why} by git")
    return problems


def missing(root, keys):
    return [k for k in keys if not os.path.exists(os.path.join(root, k.private))]
