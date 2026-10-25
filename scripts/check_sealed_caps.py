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
"""The caps a sealed manifest grants, held to what tools/nix/capsules.json says.

The kernel embeds each capsule's signed manifest from nonos-data/trust, and
the manifest's required and optional caps are what the capsule is granted.
A Capsule.mk that narrows or widens a grant changes capsules.json at once,
but the sealed manifest only at the next seal; until then the image would
ship the old grant under the new code. Given the slugs a kernel ships, this
fails when any of their sealed manifests says other caps, so the seal has to
run first. --self-test builds manifests to check the decode and the verdict.
"""

import argparse
import json
import struct
import sys
from pathlib import Path

CAPSULES = Path("tools/nix/capsules.json")
SCHEMA = 3


class Undecodable(Exception):
    pass


def caps(blob):
    """(required, optional) from a manifest's header (schema 3, big-endian)."""
    try:
        if struct.unpack_from(">H", blob)[0] != SCHEMA:
            raise Undecodable("not a schema 3 manifest")
        at = 2 + 32
        at += 1 + blob[at]
        at += 12
        at += 1 + blob[at]
        at += 32
        return struct.unpack_from(">QQ", blob, at)
    except (struct.error, IndexError) as e:
        raise Undecodable(f"cut short: {e}") from e


def stale(entries, read):
    """[(slug, sealed, wanted)] for each entry whose manifest grants other caps."""
    out = []
    for e in entries:
        blob = read(e["manifest"])
        if blob is None:
            continue
        sealed = caps(blob)
        wanted = (int(e["required_caps"], 16), int(e["optional_caps"], 16))
        if sealed != wanted:
            out.append((e["slug"], sealed, wanted))
    return out


def _manifest(required, optional):
    ns, triple = b"systems.nonos.x", b"x86_64-nonos-user"
    head = struct.pack(">H", SCHEMA) + bytes(32) + bytes([len(ns)]) + ns + bytes(12)
    return head + bytes([len(triple)]) + triple + bytes(32) + struct.pack(">QQ", required, optional)


def self_test():
    blobs = {"a.bin": _manifest(0x18, 0x100), "b.bin": _manifest(0x38, 0)}
    entries = [{"slug": "a", "manifest": "a.bin", "required_caps": "0x18", "optional_caps": "0x100"},
               {"slug": "b", "manifest": "b.bin", "required_caps": "0x10000038", "optional_caps": "0x0"},
               {"slug": "c", "manifest": "c.bin", "required_caps": "0x8", "optional_caps": "0x0"}]
    got = stale(entries, blobs.get)
    assert got == [("b", (0x38, 0), (0x10000038, 0))], got
    for bad in (b"", b"\x00\x02" + bytes(80), _manifest(1, 2)[:-3]):
        try:
            caps(bad)
        except Undecodable:
            continue
        raise AssertionError(f"{bad[:4]!r} decoded")
    print("check_sealed_caps: self-test passed")
    return 0


def main(argv=None):
    ap = argparse.ArgumentParser(description=__doc__.split("\n", 1)[0])
    ap.add_argument("slugs", nargs="*", help="the capsules a kernel ships")
    ap.add_argument("--self-test", action="store_true")
    a = ap.parse_args(argv)
    if a.self_test:
        return self_test()
    entries = [e for e in json.loads(CAPSULES.read_text()) if e["slug"] in set(a.slugs)]

    def read(path):
        p = Path(path)
        return p.read_bytes() if p.exists() else None

    try:
        found = stale(entries, read)
    except Undecodable as e:
        print(f"sealed caps: a manifest cannot be read: {e}", file=sys.stderr)
        return 2
    for slug, sealed, wanted in found:
        print(f"sealed caps: {slug}: the sealed manifest grants {sealed[0]:#x}/{sealed[1]:#x}, "
              f"Capsule.mk says {wanted[0]:#x}/{wanted[1]:#x}; reseal it", file=sys.stderr)
    return 1 if found else 0


if __name__ == "__main__":
    sys.exit(main())
