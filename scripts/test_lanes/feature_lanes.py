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
"""A crate in a lane may ask for its tests to run again with a feature on:

    [package.metadata.nonos]
    feature-lanes = ["devnet"]

Each is the lane proofs-<name>-<feature>, written to test-lanes.json as
"<dir>+<feature>": its test-args and --features <feature>. A test behind a
feature no default build turns on runs nowhere else."""

from .lanes import marked, marker, proofs

SEP = "+"


def in_lane(tree, d):
    return proofs(tree, d) or marker(tree, d).get("test-lane") is True


def feature_lanes(tree):
    """<dir>+<feature> -> cargo test arguments, for every feature lane asked."""
    out = {}
    for d in tree.crates:
        wanted = marker(tree, d).get("feature-lanes")
        if not in_lane(tree, d) or not isinstance(wanted, list):
            continue
        args = marker(tree, d).get("test-args", "")
        for f in wanted:
            if isinstance(f, str):
                out[f"{d}{SEP}{f}"] = f"{args} --features {f}".strip()
    return out


def table(tree):
    """Every lane test-lanes.json names: the marked crates and their feature lanes."""
    return {**marked(tree), **feature_lanes(tree)}


def feature_misuse(tree):
    """A feature-lanes marker the lane cannot act on as written."""
    for d in tree.crates:
        wanted = marker(tree, d).get("feature-lanes")
        if wanted is None:
            continue
        known = tree.manifests[d].get("features", {})
        why = ("feature-lanes must be a list of feature names"
               if not isinstance(wanted, list) or not all(isinstance(f, str) for f in wanted) else
               "feature-lanes on a crate in no lane" if not in_lane(tree, d) else
               next((f"feature-lanes names {f}, which [features] does not declare"
                     for f in wanted if f not in known), None))
        if why:
            yield f"{d}: {why}"
