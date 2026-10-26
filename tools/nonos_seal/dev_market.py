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

"""A development image's market index: the schema 3 test catalogue in
tools/devnet/market, so its Marketplace lists priced entries and the paid
paths can be walked. It is signed with the throwaway operator key
nonos-dev-image made in its copy, which only that copy's market capsule
trusts. A release never reads it (scripts/check_dev_catalogue.py)."""

import json
import os

from . import capsules, keys

SOURCE = "tools/devnet/market/index.json"
FILLED = "target/market/dev-index.json"
# The catalogue cannot name the key that will sign it: each copy makes its own.
PLACEHOLDER = "OPERATOR"
DEV_TREE = os.path.join("target", "dev", "tree")


def in_dev_tree():
    """Whether this is nonos-dev-image's copy: a repository of its own at
    <checkout>/target/dev/tree, never the checkout that holds the real keys."""
    return os.path.realpath(os.getcwd()).endswith(os.sep + DEV_TREE) and os.path.isdir(".git")


def chosen():
    return capsules.DEVELOPMENT and in_dev_tree()


def fill(doc, operator_hex, serial):
    """The catalogue as the operator signs it: every listing published by the
    copy's operator key, at this seal's serial."""
    for entry in doc["entries"]:
        if entry["publisher_pubkey"] == PLACEHOLDER:
            entry["publisher_pubkey"] = operator_hex
    doc["serial"] = int(serial)
    doc["published_at_ms"] = int(serial) * 1000
    return doc


def seal(step, tools, out, serial):
    with open(keys.OPERATOR_PUB, "rb") as f:
        key = f.read()
    # The checkout's operator key signing fake listings would make them ones a
    # release trusts; the copy's key is a throwaway, so the two must differ.
    checkout = os.path.join(os.getcwd(), "..", "..", "..", keys.OPERATOR_PUB)
    if os.path.isfile(checkout) and open(checkout, "rb").read() == key:
        raise SystemExit("  the development catalogue is never signed with the checkout's operator key")
    with open(SOURCE) as f:
        doc = fill(json.load(f), key.hex(), serial)
    os.makedirs(os.path.dirname(FILLED), exist_ok=True)
    with open(FILLED, "w") as f:
        json.dump(doc, f, indent=2)
    step.run("python3", "tools/nonos-market-sign-releases", "--cli", tools.market_index, "--index", FILLED,
             "--key-file", keys.OPERATOR_SEED, "--operator-pubkey", key.hex())
    step.run(tools.market_index, "sign", "--in", FILLED, "--key-file", keys.OPERATOR_SEED,
             "--pubkey", key.hex(), "--out", out)
    step.run(tools.market_index, "verify", "--in", out, "--pubkey", key.hex())
