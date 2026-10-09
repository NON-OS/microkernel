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
import shutil
import subprocess

from . import keys
from .run import say

INDEX = "nonos-data/market/index.bin"
CATALOGUE = "nonos-data/models/catalogue.bin"
USERLAND = "target/linux-userland"


def committed(*paths):
    """Whether every path is tracked and the same as at HEAD."""
    for path in paths:
        tracked = subprocess.run(["git", "ls-files", "--error-unmatch", "--", path],
                                 capture_output=True).returncode == 0
        same = subprocess.run(["git", "diff", "--quiet", "HEAD", "--", path]).returncode == 0
        if not (os.path.isfile(path) and tracked and same):
            return False
    return True


def place_userland(linux_out):
    """The index names the Qwen runner by the BLAKE3 of the binary the store
    carries, which the catalogue tool reads from target/linux-userland."""
    shutil.rmtree(USERLAND, ignore_errors=True)
    os.makedirs(USERLAND)
    # Every file, with the data some programs keep in a directory of their
    # own (Tcl's script library), as plain writable copies: the output's
    # links are followed and the store's read-only modes are not kept.
    for d, _, names in os.walk(linux_out, followlinks=True):
        rel = os.path.relpath(d, linux_out)
        os.makedirs(os.path.join(USERLAND, rel), exist_ok=True)
        for name in sorted(names):
            shutil.copyfile(os.path.join(d, name), os.path.join(USERLAND, rel, name))


# What the market index and the model catalogue are made from. Their serial
# is the time of the last commit that changed any of these, not of HEAD, so a
# commit that changes none of them signs the same two files, and the capsules
# that embed them keep their bytes and their enrollment.
SOURCES = [
    "userland/capsule_market", "userland/capsule_linux/src/linux/file/models",
    "tools/nonos-market-index", "tools/nonos-market-catalogue", "tools/nonos_market_catalogue",
    "tools/nonos-market-sign-releases", "tools/nonos-qwen-tier.py", "tools/nonos_qwen_tier",
    "tools/nix/userland.nix", "tools/nix/sources.txt", "nonos-mk",
]


def serial():
    """The commit time of the last change to SOURCES, or of HEAD when the
    history does not reach one."""
    def last(*paths):
        return subprocess.run(["git", "log", "-1", "--format=%ct", "--", *paths],
                              capture_output=True, text=True, check=True).stdout.strip()
    return last(*[p for p in SOURCES if os.path.exists(p)]) or last()


def seal(step, tools, linux_out, serial, mirror, package_mirror="", packages="", required=False):
    # An image people will use needs both catalogues: without them the
    # Marketplace lists nothing and no Qwen tier can be fetched. Only a
    # development image may go without; this asks only whether the key is
    # there, never what it holds.
    if not os.path.isfile(keys.OPERATOR_SEED):
        # The operator signs deterministically from SOURCES, so the index and
        # catalogue a seal committed are the ones it would sign again for an
        # unchanged tree. A seal without the seed (a CI runner's) keeps those,
        # under the operator key in the tree: it signs nothing, so it can
        # forge nothing, and the image checks both signatures itself.
        if committed(INDEX, CATALOGUE, keys.OPERATOR_PUB):
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
    step.run("python3", "tools/nonos-market-index", "--out", INDEX, "--cli", tools.market_index,
             "--seed", keys.OPERATOR_SEED, "--pubkey", keys.OPERATOR_PUB,
             "--linux-list", "userland/capsule_market/linux-packages.txt",
             "--guest-list", "userland/capsule_market/linux-guests.json", "--serial", serial,
             # The in-tree tools the image does not carry, listed only when it
             # is built with a NONOS package mirror, their content written to
             # `packages` for ek to put on it.
             *(["--tool-list", "userland/capsule_market/linux-tools.json",
                "--package-mirror", package_mirror, "--package-out", packages] if package_mirror else []),
             env={"SOURCE_DATE_EPOCH": serial})
    step.run("python3", "tools/nonos-qwen-tier.py", "catalogue", "--out", CATALOGUE,
             "--seed", keys.OPERATOR_SEED, "--pubkey", keys.OPERATOR_PUB, "--serial", serial,
             *(["--nonos-mirror", mirror] if mirror else []))
    keys.stage(INDEX, CATALOGUE)
