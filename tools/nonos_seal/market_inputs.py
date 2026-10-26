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

"""What phase 1 signs from: whether the signed files are the ones at HEAD,
the userland the index names by hash, and the serial both carry."""

import os
import shutil
import subprocess

USERLAND = "target/linux-userland"
# The logo rasters a maintainer committed (tools/nonos_market_catalogue/logos.py),
# which the market capsule embeds and the index signs (market_logos.py).
LOGOS = "nonos-data/market/logos"


def committed(*paths):
    """Whether every path, a file or a directory of them, is tracked and the
    same as at HEAD, with nothing untracked beside it."""
    for path in paths:
        tracked = subprocess.run(["git", "ls-files", "--error-unmatch", "--", path],
                                 capture_output=True).returncode == 0
        same = subprocess.run(["git", "diff", "--quiet", "HEAD", "--", path]).returncode == 0
        loose = subprocess.run(["git", "ls-files", "--others", "--", path],
                               capture_output=True, text=True).stdout.strip()
        if not (os.path.exists(path) and tracked and same and not loose):
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
    "tools/nix/userland.nix", "tools/nix/sources.txt", "nonos-mk", LOGOS,
    "HANDOFF/catalogue/logos-licences.md",
    # The catalogue's recipes, its policy encoder, the Alpine keys it checks
    # under and the release the device installs from.
    "tools/nonos_linux_pack", "userland/capsule_linux/keys/alpine",
    "userland/capsule_linux/src/linux/install/run.rs",
]


def serial():
    """The commit time of the last change to SOURCES, or of HEAD when the
    history does not reach one."""
    def last(*paths):
        return subprocess.run(["git", "log", "-1", "--format=%ct", "--", *paths],
                              capture_output=True, text=True, check=True).stdout.strip()
    return last(*[p for p in SOURCES if os.path.exists(p)]) or last()
