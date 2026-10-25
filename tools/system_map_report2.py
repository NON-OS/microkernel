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
"""How authority is spread, who holds the scarce kind, and the TSV."""

from system_map_consts import CAPABILITIES, CYAN, DIM, EM_MACHINES, GREEN, OFF, SCARCE, YELLOW
from system_map_out import heading, out

def report_distribution(capsules):
    heading("AUTHORITY DISTRIBUTION")
    built = [c for c in capsules if c.bytes]
    total = len(built)
    counts = []
    for bit, name in CAPABILITIES:
        holders = [c for c in built if c.mask & bit]
        if holders:
            counts.append((len(holders), name))
    counts.sort(reverse=True)
    width = 34
    for count, name in counts:
        filled = max(1, int(round(width * count / total))) if total else 0
        colour = YELLOW if name in SCARCE else CYAN
        bar = colour + "#" * filled + OFF + DIM + "." * (width - filled) + OFF
        out("  {:<17} {} {:>3} of {}".format(name, bar, count, total))


def report_scarce(capsules):
    heading("SCARCE AUTHORITY")
    out("  " + DIM + "Authority that acts on something its holder does not own." + OFF)
    out("")
    built = [c for c in capsules if c.bytes]
    for bit, name in CAPABILITIES:
        if name not in SCARCE:
            continue
        holders = sorted(c.slug for c in built if c.mask & bit)
        if not holders:
            continue
        colour = GREEN if len(holders) <= 2 else YELLOW
        out("  {:<17} {}{}{}".format(name, colour, ", ".join(holders), OFF))


def report_tsv(capsules):
    print("\t".join(["slug", "namespace", "bin", "caps_mask", "service",
                     "bytes", "machine", "attested", "capabilities"]))
    for capsule in sorted(capsules, key=lambda c: c.slug):
        print("\t".join([
            capsule.slug,
            capsule.namespace,
            capsule.bin_name,
            hex(capsule.mask),
            capsule.service,
            str(capsule.bytes),
            EM_MACHINES.get(capsule.machine, str(capsule.machine)),
            "yes" if capsule.attested else "no",
            " ".join(capsule.caps),
        ]))
