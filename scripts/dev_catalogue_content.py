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
"""What the development catalogue holds, and how the seal's choice of it
behaves when run: only a development seal in nonos-dev-image's copy takes it,
and that copy never signs it with the checkout's operator key."""

import contextlib
import os
import re
import sys
import types
from urllib.parse import urlparse

SOURCE = "tools/devnet/market/index.json"
# Placeholders no one holds a key for (tools/devnet/market/README.md).
PAYEE = re.compile(r"^0x(dd){18}[0-9a-f]{4}$")
BINDING = "ee" * 65


def catalogue(doc):
    out = [] if doc.get("schema_version") == 3 else ["the catalogue is not schema 3"]
    entries = doc.get("entries", [])
    for e in entries:
        lid, c = e.get("listing_id", "?"), e.get("commerce")
        if e.get("publisher_pubkey") != "OPERATOR":
            out.append(f"{lid}: names a publisher key; only the copy's operator key may publish it")
        if "qwen" in (lid + e.get("name", "")).lower():
            out.append(f"{lid}: a Qwen tier; the release index lists those")
        if not c:
            out.append(f"{lid}: no commerce block")
            continue
        if not PAYEE.match(c.get("payee", "")) or c.get("binding_sig") != BINDING or c.get("bond_id"):
            out.append(f"{lid}: payee, binding or bond is not the documented placeholder")
        hosts = [urlparse(r.get("package_url", "")).hostname or "" for r in e.get("releases", [])]
        if not hosts or not all(h.endswith(".invalid") for h in hosts):
            out.append(f"{lid}: a package outside the reserved .invalid domain")
    modes = [(e.get("commerce") or {}).get("settle_mode") for e in entries]
    priced = [e for e in entries if (e.get("commerce") or {}).get("settle_mode") in ("voucher", "escrow")
              and int(e["commerce"].get("unit_price", "0")) > 0]
    return out + ([] if priced else ["no priced listing"]) + ([] if "free" in modes else ["no free listing"])


def load(root, mutate=lambda s: s):
    """tools/nonos_seal/dev_market.py as the seal imports it, its source changed by `mutate`."""
    sys.dont_write_bytecode = True
    sys.path.insert(0, str(root / "tools"))
    from nonos_seal import capsules
    mod = types.ModuleType("nonos_seal.dev_market_under_test")
    mod.__package__ = "nonos_seal"
    exec(compile(mutate((root / "tools/nonos_seal/dev_market.py").read_text()), "dev_market.py", "exec"),
         mod.__dict__)
    return mod, capsules


@contextlib.contextmanager
def inside(path):
    was = os.getcwd()
    os.chdir(path)
    try:
        yield
    finally:
        os.chdir(was)
