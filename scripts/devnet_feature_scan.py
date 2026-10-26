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
"""The rules scripts/check_devnet_feature.py holds the tree to, one per kind
of file; each answers the reasons a file breaks them, or nothing."""

import json
import re

WORD = re.compile(r"\bdevnet\b")
# The release ELF check names the strings it looks for, one per line.
NAMES = "tools/nix/release_names.nix"
NAME_LINE = re.compile(r'^\s*"[^"\s]+"\s*$')


def lines(text):
    """Each line with its # comment cut, numbered from 1."""
    return [(n, line.split("#", 1)[0]) for n, line in enumerate(text.split("\n"), 1)]


def cargo(path, text):
    """A dependency may not turn devnet on; only a feature named devnet may."""
    out, section, feature = [], "", None
    for n, line in lines(text):
        head = re.match(r"^\s*\[+([^\]]+)\]+\s*$", line)
        if head:
            section, feature = head.group(1).strip(), None
            continue
        named = re.match(r"^\s*([A-Za-z0-9_-]+)\s*=", line)
        if section == "features" and named:
            feature = named.group(1)
        if not WORD.search(line):
            continue
        if "dependencies" in section:
            out.append(f"{path}:{n}: a dependency turns devnet on")
        elif section == "features" and feature != "devnet":
            out.append(f"{path}:{n}: feature {feature} turns devnet on")
    return out


def capsule_mk(path, text):
    """Only CAPSULE_DEV_CARGO_FEATURES, the development set's, names devnet."""
    return [f"{path}:{n}: devnet outside CAPSULE_DEV_CARGO_FEATURES" for n, line in lines(text)
            if WORD.search(line) and not line.lstrip().startswith("CAPSULE_DEV_CARGO_FEATURES ")]


def catalogue(path, text, allowed):
    """Only the keys allowed name devnet: capsules-dev.json's dev_cargo_features."""
    return [f"{path}: {e.get('slug')}.{k} names devnet" for e in json.loads(text) for k, v in e.items()
            if k not in allowed and isinstance(v, str) and WORD.search(v)]


def anywhere(path, text):
    """A profile, image, lane or workflow never names devnet; the release ELF
    check names only the strings it looks for."""
    names = path == NAMES
    return [f"{path}:{n}: names devnet" for n, line in lines(text)
            if WORD.search(line) and not (names and NAME_LINE.match(line))]
