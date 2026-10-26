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
"""Where a release could pick the development catalogue (tools/devnet/market),
and the lines that keep it from doing so."""

import re

# A path to the catalogue, however it is spelt: "tools/devnet/market",
# os.path.join("tools", "devnet", "market"), and the seal module that reads it.
NAMES = re.compile(r"devnet\W{1,6}market|\bdev_market\b")

# What an image, a release seal or a release capsule build reads. The host
# proofs (userland/*_proofs, nonos-mk/tests) build no image, and the crate
# input table lists their files without reading them.
PLACES = ("Makefile", "nonos.toml", "flake.nix", "mk/", "nonos-mk/", "nonos-ci/", ".github/",
          "tools/nix/", "tools/nonos_seal/", "tools/nonos-market-", "tools/nonos_market_catalogue/",
          "tools/nonos-qwen-tier.py", "tools/nonos_qwen_tier/", "userland/", "nonos-data/")
UNREAD = ("nonos-mk/tests/", "tools/nix/inputs.json")
PROOFS = re.compile(r"^userland/[^/]+_proofs/")
# The seal's one gate, and the module behind it.
GATE = "tools/nonos_seal/market.py"
READER = "tools/nonos_seal/dev_market.py"

# Each place a release could take the development catalogue still refuses it.
PINS = [
    (GATE, "    if dev_market.chosen():\n        dev_market.seal(step, tools, INDEX, serial)\n    else:\n"),
    (READER, "    return capsules.DEVELOPMENT and in_dev_tree()\n"),
    (READER, 'endswith(os.sep + DEV_TREE) and os.path.isdir(".git")'),
    (READER, 'if os.path.isfile(checkout) and open(checkout, "rb").read() == key:'),
    ("tools/nonos_seal/__main__.py", '    capsules.DEVELOPMENT = not cfg["release"]\n'),
    ("tools/nonos_seal/capsules.py", "\nDEVELOPMENT = False\n"),
    ("tools/nonos-dev-image", 'TREE = ROOT / "target/dev/tree"\n'),
]


def pinned(files):
    return [f"{p}: no longer holds: {want.strip()}" for p, want in PINS if want not in files.get(p, "")]


def watched(path):
    return (path.startswith(PLACES) and not path.startswith(UNREAD) and not PROOFS.match(path)
            and path != READER)


def named(path, text):
    """A release path that names the development catalogue. The gate may name
    the module, on the lines PINS holds and its import."""
    if not watched(path):
        return []
    out = []
    for n, line in enumerate(text.split("\n"), 1):
        allowed = path == GATE and line.strip() in (
            "from . import dev_market, keys", "if dev_market.chosen():",
            "dev_market.seal(step, tools, INDEX, serial)")
        if NAMES.search(line) and not allowed:
            out.append(f"{path}:{n}: names the development catalogue")
    return out
