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

"""Phase 1: the two files the market operator signs and two capsules embed.
The market capsule carries the signed package index and the model fetcher the
signed model catalogue, so both are signed before any capsule is built and
committed under nonos-data/, where the flake reads them. Without the operator
key both stay empty, as they always have."""

import os

from . import dev_market, keys, market_logos
from .market_inputs import place_userland
from .run import say

INDEX = "nonos-data/market/index.bin"
CATALOGUE = "nonos-data/models/catalogue.bin"


def seal(step, tools, linux_out, serial, mirror, package_mirror="", packages="", required=False):
    # An image people will use needs both catalogues: without them the
    # Marketplace lists nothing and no Qwen tier can be fetched. Only a
    # development image may go without; this asks only whether the key is
    # there, never what it holds.
    if not os.path.isfile(keys.OPERATOR_SEED):
        # The operator signs deterministically from SOURCES, so the index and catalogue a seal
        # committed are the ones it would sign again for an unchanged tree. A seal without the
        # seed (a CI runner's) keeps those, and the logos they sign, under the operator key in the
        # tree: it signs nothing, so it can forge nothing, and the image checks both signatures.
        if market_logos.kept(INDEX, CATALOGUE, keys.OPERATOR_PUB):
            say("  no market operator key: kept the index and the model catalogue the operator signed")
            place_userland(linux_out)
            return
        if required:
            raise SystemExit(f"  no market operator key at {keys.OPERATOR_SEED}: this image would ship an empty "
                             "Marketplace and no Qwen tier to fetch, so it is not sealed")
        say("  no market operator key: the index and the model catalogue stay empty (a development image)")
        return
    place_userland(linux_out)
    for path in (INDEX, CATALOGUE):
        os.makedirs(os.path.dirname(path), exist_ok=True)
    # A development image in nonos-dev-image's copy lists the schema 3 test
    # catalogue instead, under the copy's throwaway key; a release never does.
    if dev_market.chosen():
        dev_market.seal(step, tools, INDEX, serial)
    else:
        step.run("python3", "tools/nonos-market-index", "--out", INDEX, "--cli", tools.market_index,
                 "--seed", keys.OPERATOR_SEED, "--pubkey", keys.OPERATOR_PUB,
                 "--linux-list", "userland/capsule_market/linux-packages.txt",
                 "--guest-list", "userland/capsule_market/linux-guests.json", "--serial", serial,
                 # The catalogue's apps: from Alpine, under the keys the device pins, each with its box.
                 "--catalogue", "userland/capsule_market/linux-catalogue.json",
                 # The in-tree tools the image does not carry, listed only with a NONOS
                 # package mirror, their content written to `packages` for ek to put on it.
                 *(["--tool-list", "userland/capsule_market/linux-tools.json",
                    "--package-mirror", package_mirror, "--package-out", packages] if package_mirror else []),
                 env={"SOURCE_DATE_EPOCH": serial})
    step.run("python3", "tools/nonos-qwen-tier.py", "catalogue", "--out", CATALOGUE,
             "--seed", keys.OPERATOR_SEED, "--pubkey", keys.OPERATOR_PUB, "--serial", serial,
             *(["--nonos-mirror", mirror] if mirror else []))
    market_logos.agree()
    keys.stage(INDEX, CATALOGUE)
