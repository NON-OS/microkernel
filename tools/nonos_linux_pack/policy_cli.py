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
"""nonos-linux-pack policy: a market release's guest_policy (plan/30).

Written from the program, its services, the calls its runs measured
(`nonos-linux-pack syscalls --from-log`), optionally the destinations it may
reach, and the release's publisher key, app and release names, from which
listing_id and release_id follow by marketplace_abi's rule. It prints the
256 bytes as hex, which the release carries; it signs nothing. The release is
signed by its publisher, and the installer vouches the policy (plan/30).
"""

import shutil
import subprocess
import sys
from pathlib import Path

from .blake3 import blake3
from .dests import Refused, dests_hash, dests_list
from .policy import encode, listing_id, mask_of, release_id
from .policy_args import parser


def program_hash(path: Path) -> bytes:
    """b3sum when it is installed, as the signing tools use; else the same
    hash computed here, slowly."""
    if shutil.which("b3sum"):
        done = subprocess.run(["b3sum", "--no-names", "--raw", str(path)],
                              stdout=subprocess.PIPE, check=True)
        return done.stdout[:32]
    return blake3(path.read_bytes())


def main(argv) -> int:
    a = parser(__doc__).parse_args(argv)
    try:
        publisher = bytes.fromhex(a.publisher)
        calls = bytes.fromhex(a.calls.removeprefix("calls="))
        listing = dests_list(a.dests.read_text().replace(",", "\n").splitlines()) if a.dests else None
        mask = mask_of(a.mask)
        if listing and not mask & 2:
            raise ValueError("a destination list needs NET_ANON")
        if len(publisher) != 32:
            raise ValueError("the publisher key is not 32 bytes")
        lid = listing_id(publisher, a.app)
        rid = release_id(lid, a.release)
        hashed = dests_hash(listing) if listing else bytes(32)
        policy = encode(program_hash(a.program), mask, lid, rid, hashed, calls)
    except (ValueError, Refused, OSError) as why:
        print(f"linux-pack policy: {why}", file=sys.stderr)
        return 1
    print(f"listing_id={lid.hex()}\nrelease_id={rid.hex()}")
    if listing:
        print(f"dests={listing.hex()}")
    print(f"guest_policy={policy.hex()}")
    if a.out:
        a.out.write_bytes(policy)
    return 0
