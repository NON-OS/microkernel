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
"""A file's theorem statements, and the names it gives a function."""

import re

THEOREM = re.compile(r'^(?:private\s+)?theorem\s+(\S+)(.*?):=', re.S | re.M)
RENAMING = re.compile(r'\b(\w+)\s*→\s*(\w+)')
ALIAS = re.compile(
    r'^(?:private\s+|noncomputable\s+)*(?:abbrev|def)\s+(\w+)([^\n]*(?:\n[ \t]+[^\n]*)*)',
    re.M)


def statements(text):
    """The statements of a file's theorems, wrappers left out.

    A name in a proof, an `open ... renaming` line, an `attribute` list or a
    `#print axioms` line is not a property of the function. Six functions once
    counted as substantive from those lines alone.
    """
    return [sig for name, sig in THEOREM.findall(text)
            if not re.match(r'the_\w+_wrapper_is_its_method$', name)]


def aliases(texts, leaf):
    """Names a file gives the function: `open ... renaming` and `abbrev`."""
    names = {leaf}
    word = re.compile(r'\b%s\b' % re.escape(leaf))
    for t in texts:
        names.update(b for a, b in RENAMING.findall(t) if a == leaf)
        names.update(m.group(1) for m in ALIAS.finditer(t) if word.search(m.group(2)))
    return names
