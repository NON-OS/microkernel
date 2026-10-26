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
"""The verdict: every test runs in a lane or is listed, and every listed line
still names a test no lane runs."""

from .baseline import BASELINE, drift, read
from .feature_lanes import feature_misuse, table
from .lanes import misuse
from .unrun import bearing, unrun

SAY = {
    "crate": "{path} has tests in no lane ({n} files, first {first}): mark it test-lane = true "
             "under [package.metadata.nonos], or list it in " + str(BASELINE),
    "orphan": "{path} holds tests that no target of {crate} reaches: add the mod line that "
              "compiles it, or list it as `{path} orphan <owner> <reason>`",
    "dropped": "{path} holds tests of {crate} that its test-args {args!r} skip: run it, or "
               "list it with the reason",
}

KIND = {"crate": "a crate", "orphan": "an orphan", "dropped": "a target its test-args skip"}


def stale(tree, lane, path):
    """Why a listed path no longer needs its line."""
    if path in lane:
        return "it is in a lane now"
    if path not in tree.manifests and path not in tree.files:
        return "it is gone"
    crate = path if path in tree.manifests else tree.owner(path)
    if crate != path and crate not in lane:
        return f"its crate {crate} is in no lane, so the list names the crate, not the file"
    files = tree.owned.get(path, []) if path in tree.manifests else [path]
    if any(bearing(tree, f, crate) for f in files):
        return "none of its tests is left out of a lane now"
    return "it has no tests"


def verdict(tree):
    """Every problem with the tree's tests, its markers and its list."""
    lane, found = unrun(tree)
    listed, problems = read(tree.root)
    problems += list(misuse(tree)) + list(feature_misuse(tree)) + drift(tree.root, table(tree))
    for path, (kind, crate, files) in sorted(found.items()):
        cls = listed.get(path, ("",))[0]
        if not cls:
            problems.append(SAY[kind].format(path=path, crate=crate, n=len(files), first=files[0],
                                             args=lane.get(crate, "")))
        elif (cls == "orphan") != (kind == "orphan"):
            problems.append(f"{BASELINE} gives {path} class {cls}, but it is {KIND[kind]}")
    problems += [f"{BASELINE} lists {p}, which is stale: {stale(tree, lane, p)}; drop the line"
                 for p in sorted(set(listed) - set(found))]
    return problems, lane, listed
