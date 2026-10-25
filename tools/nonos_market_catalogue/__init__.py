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
"""Pieces of tools/nonos-market-catalogue, kept in files of their own."""

import os
import shutil


def load_tests(loader, standard_tests, pattern):
    """The suite, once b3sum is found: listings carry its BLAKE3 digests, so
    without it the suite stops in one line rather than failing test by test."""
    if shutil.which("b3sum") is None:
        raise SystemExit("the catalogue suite needs b3sum (BLAKE3); install it")
    standard_tests.addTests(loader.discover(os.path.dirname(__file__), pattern or "test*.py"))
    return standard_tests
