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

"""The attack suite's rules: every attack a boot must show refused, and the
isolation matrix rows each refusal has to prove. Data only; judge.py
applies it, rows.py holds the matrix rows and dropped.py the rows dropped."""

# The report format, raised when a record changes shape.
REPORT_VERSION = 1

# Every attack a run must show refused, by the boot that prints it (attest_refusal,
# capsule_attack, linux_guests). A run judged for some boots is judged for theirs.
BOOTS = {
    "attest": ("trailer-flip", "trailer-extra-cap", "trailer-kernel-kind", "trailer-stale-epoch"),
    "capsule": ("cap-escape", "foreign-memory", "service-squat", "held-escape", "deputy-escape",
                "reply-twice", "fault-containment"),
    "linux": ("linux-fs-escape", "linux-foreign-memory", "linux-raw-syscall"),
}
ATTACKS = tuple(a for attacks in BOOTS.values() for a in attacks)

from .rows import ROWS  # noqa: E402,F401  (judge, mutants and selfcheck read it here)
