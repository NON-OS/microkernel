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

"""What the first boot left behind: on the disk it booted from, and in what
the second boot printed."""

import importlib.machinery
import importlib.util
from pathlib import Path

from .memscan import count

ROOT = Path(__file__).resolve().parents[2]
_loader = importlib.machinery.SourceFileLoader(
    "nonos_amnesic_check", str(ROOT / "tools/nonos-amnesic-check"))
_amnesic = importlib.util.module_from_spec(importlib.util.spec_from_loader(_loader.name, _loader))
_loader.exec_module(_amnesic)


def store_rows_added(before_image, after_image):
    """The store rows the boot added or changed: an amnesic boot adds none."""
    before = set(_amnesic.rows(str(before_image)))
    return [r.split(" ", 1)[0] for r in _amnesic.rows(str(after_image)) if r not in before]


def disk_holds(image, secret):
    """How many times the secret is anywhere on the disk, sector by sector."""
    return count(image, secret)


def said(serial, secret):
    return secret in Path(serial).read_text(errors="replace")
