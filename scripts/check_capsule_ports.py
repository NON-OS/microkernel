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

"""Check that no two capsules declare the same IPC port. Every service, reply
and instance endpoint in userland/*/Capsule.mk and tools/nix/capsules.json,
and every Linux guest's pair, names a port; two owners of one port means a
message meant for one is delivered to the other."""

import argparse
import pathlib
import sys

from capsule_port_json import json_ports
from capsule_port_sources import capsule_ports, guest_ports

ROOT = pathlib.Path(__file__).resolve().parent.parent


def clashes(root):
    owners = {}
    for mk in sorted(root.glob("userland/*/Capsule.mk")):
        for port, who in capsule_ports(mk):
            owners.setdefault(port, []).append(who)
    for port, who in (*guest_ports(root), *json_ports(root)):
        owners.setdefault(port, []).append(who)
    return {p: w for p, w in owners.items() if shared(w)}, len(owners)


def shared(who):
    # A guest makefile's held ports ("<file>:*") are for the guests that file
    # makes, so one of those guests is the holder's user, not a second owner.
    owners = {x.split()[0] for x in who}
    holders = {o[:-2] for o in owners if o.endswith(":*")}
    users = owners - {h + ":*" for h in holders}
    if len(users) > 1 or len(holders) > 1:
        return True
    return any(u.split(":")[0] not in holders for u in users) if holders else False


def main():
    ap = argparse.ArgumentParser(description=__doc__.split("\n", 1)[0])
    ap.add_argument("--self-test", action="store_true", help="prove the check bites")
    if ap.parse_args().self_test:
        from capsule_ports_selftest import self_test
        return self_test(clashes) or 0
    bad, total = clashes(ROOT)
    for port, who in sorted(bad.items()):
        print(f"capsule ports: {port} is declared by " + "; ".join(who), file=sys.stderr)
    print(f"capsule ports: {total} ports declared, {len(bad)} shared")
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main())
