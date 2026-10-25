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

"""NONOS_QEMU_EXTRA: words added to QEMU's command line for one run, such as
a device trace. Off by default: unset or empty, the line is unchanged.

  NONOS_QEMU_EXTRA="-msg timestamp=on -trace pci_nvme_rw" nix run .#qemu
"""

import os
import shlex

VAR = "NONOS_QEMU_EXTRA"


def words(env=None):
    """The words, in order; a quoted word stays one word; none when unset."""
    return shlex.split((os.environ if env is None else env).get(VAR, ""))
