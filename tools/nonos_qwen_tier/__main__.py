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
"""The command line: list, fetch, plan, catalogue, mirror and show."""

import argparse
import os
import time

from .catalogue_file import write
from .download import fetch
from .mirror import mirror
from .pins import TOOLS, pins
from .plan import plan
from .show import show
from .upstream import upstream

KEYS = os.path.join(TOOLS, "..", ".keys", "marketplace_operator_ed25519")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("verb", choices=["list", "fetch", "plan", "catalogue", "mirror", "show"])
    ap.add_argument("tiers", nargs="*")
    ap.add_argument("--dir", default=".")
    ap.add_argument("--image")
    ap.add_argument("--fresh", action="store_true")
    ap.add_argument("--all", action="store_true", help="mirror: every tier")
    ap.add_argument("--out", help="catalogue: the file to write")
    ap.add_argument("--nonos-mirror", default="", help="base URL of the NONOS repository")
    ap.add_argument("--seed", default=KEYS + ".seed", help="operator Ed25519 seed, 32 bytes")
    ap.add_argument("--pubkey", default=KEYS + ".pub", help="operator public key, 32 bytes")
    ap.add_argument("--serial", type=int, default=int(time.time()))
    a, table = ap.parse_args(), pins()
    if a.verb == "list":
        for tier, files in table.items():
            print(f"{tier:13} {sum(s for _, s, _ in files) / 1e9:5.2f} GB  "
                  + " ".join(n for n, _, _ in files))
        return
    if a.verb == "show":
        return show(a.tiers[0] if a.tiers else ap.error("show FILE"), a.pubkey)
    if a.verb == "catalogue":
        return write(a.out or ap.error("catalogue needs --out"), table, a.nonos_mirror,
                     a.serial, a.seed, a.pubkey)
    chosen = list(table) if a.all and a.verb == "mirror" else list(dict.fromkeys(a.tiers))
    if not chosen or any(t not in table for t in chosen):
        ap.error(f"name one or more tiers of: {' '.join(table)}")
    if a.verb == "mirror":
        if not a.nonos_mirror:
            ap.error("mirror needs --nonos-mirror, the base URL the files will be served from")
        return mirror(table, chosen, a.dir, a.nonos_mirror, a.serial, a.seed, a.pubkey)
    files = [f for t in chosen for f in table[t]]
    if a.verb == "fetch":
        for name, size, digest in files:
            fetch(upstream(name), size, digest, a.dir)
        return
    plan(files, a.dir, a.image or ap.error("plan needs --image (a raw image or a disk)"), a.fresh)


if __name__ == "__main__":
    main()
