#!/usr/bin/env python3
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
"""The cfg sites under src/ and the verdict over them. Read by
check_dark_features."""

import re
from pathlib import Path

from dark_features_lanes import CARGO, CFG_SITE, FEATURE_LINE, SRC, enabled, features_table, lane_features

def cfg_sites(root):
    counts = {}
    for p in (root / SRC).rglob("*.rs"):
        for name in CFG_SITE.findall(p.read_text(errors="replace")):
            counts[name] = counts.get(name, 0) + 1
    return counts


def analyse(root):
    table = features_table((root / CARGO).read_text())
    seeds = set(table.get("default", [])) | lane_features(root)
    # tools/nonos-config lets a person select any capsule feature by slug, so
    # a capsule feature is reachable by construction.
    seeds |= {f for f in table if f.startswith("nonos-capsule-")}
    on = enabled(table, seeds)
    sites = cfg_sites(root)
    undeclared = sorted(f for f in sites if f not in table)
    dark = sorted(f for f in table if f not in on and sites.get(f, 0) > 0)
    return dark, undeclared, sites


