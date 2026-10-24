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

"""A boot's steps, read from a file in nonos-drive's words, with {secret}
standing for this run's secret."""

import sys
from pathlib import Path

from .boot import ROOT

sys.path.insert(0, str(ROOT / "userland/capsule_install/e2e"))
from steps import step  # noqa: E402


def lines(path, secret):
    for line in Path(path).read_text().splitlines():
        line = line.strip()
        if line and not line.startswith("#"):
            yield line.replace("{secret}", secret)


def drive(q, path, secret, out, serial):
    """Every step in order; False at the first that did not happen."""
    for s in lines(path, secret):
        if not step(q, s, str(out), str(serial)):
            return False
    return True


def stop_for(what):
    sys.exit(f"[ZS-TEST] stopped waiting for {what}")
