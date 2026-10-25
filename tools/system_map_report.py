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
"""The image summary and the per-capsule table."""

import os

from system_map_consts import DIM, EM_MACHINES, GREEN, OFF, RED, TARGET, TRUST, YELLOW
from system_map_out import commas, heading, mib, out

def root_hex(name):
    path = os.path.join(TRUST, "policy", name)
    try:
        with open(path, "rb") as handle:
            return handle.read().hex()
    except OSError:
        return ""


def report_roots(capsules):
    heading("IMAGE")
    built = [c for c in capsules if c.bytes]
    total = sum(c.bytes for c in built)
    attested = [c for c in built if c.attested]
    out("  {}{:<16}{} {}".format(DIM, "target", OFF, TARGET))
    out("  {}{:<16}{} {} declared, {} built, {} attested".format(
        DIM, "capsules", OFF, len(capsules), len(built), len(attested)))
    out("  {}{:<16}{} {} bytes  ({})".format(
        DIM, "capsule bytes", OFF, commas(total), mib(total)))
    machines = sorted({EM_MACHINES.get(c.machine, str(c.machine))
                       for c in built if c.machine is not None})
    colour = GREEN if len(machines) == 1 else RED
    out("  {}{:<16}{} {}{}{}".format(DIM, "architecture", OFF, colour,
                                     ", ".join(machines) or "none", OFF))
    for label, name in (("policy root", "zk_capsule_policy_root.bin"),
                        ("kernel attest", "kernel_attest_root.bin")):
        value = root_hex(name)
        if value:
            out("  {}{:<16}{} {}".format(DIM, label, OFF, value))


def report_table(capsules):
    heading("CAPSULES")
    out("  {}{:<22} {:>10} {:>6}  {}{}".format(DIM, "NAME", "BYTES", "PORT",
                                               "AUTHORITY", OFF))
    for capsule in sorted(capsules, key=lambda c: c.slug):
        if not capsule.bytes:
            continue
        mark = "" if capsule.attested else YELLOW + "  [not attested]" + OFF
        out("  {:<22} {:>10} {:>6}  {}{}".format(
            capsule.slug, commas(capsule.bytes), capsule.port,
            " ".join(capsule.caps), mark))
