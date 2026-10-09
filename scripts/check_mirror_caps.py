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
"""Fail when a kernel spawn mirror grants a capsule more than its manifest.

A capsule runs with `required | (optional & granted)` from its signed
manifest, and the spawn refuses a grant wider than the manifest
(src/security/capsule_manifest/verify/caps_bits.rs). A mask cut in
Capsule.mk and left in the mirror's `requested_caps` therefore stops that
capsule at boot, and nothing on the host said so: the capability audit holds
each mask to its code, not the mirror to the manifest. This evaluates every
`requested_caps` a spawn file names, through Capability::X.bit() terms,
serial_debug_cap(), hex literals and the u64 constants the file defines, and
fails on any bit outside the manifest's required and optional sets.

A capsule's README that quotes `CAPSULE_REQUIRED_CAPS` is held to the same
manifest, so a cut mask does not go on being described as the old one."""

import argparse
import re
import sys
import tempfile
from pathlib import Path

DEFS = "src/capabilities/types/defs.rs"


def capability_bits(root):
    bits = {}
    for m in re.finditer(r"^\s*(\w+)\s*=\s*([^,]+),", (root / DEFS).read_text(), re.M):
        expr = m.group(2).strip()
        bits[m.group(1)] = (1 << int(expr.split("<<")[1])) if "<<" in expr else int(expr, 0)
    return bits


def value(expr, bits, consts):
    v = 0
    for name in re.findall(r"Capability::(\w+)\.bit\(\)", expr):
        v |= bits[name]
    if "serial_debug_cap()" in expr:
        v |= bits["Debug"]
    for name in re.findall(r"\b([A-Z][A-Z0-9_]+)\b", expr):
        v |= consts.get(name, 0)
    for lit in re.findall(r"\b0x[0-9a-fA-F]+\b", expr):
        v |= int(lit, 16)
    return v


def manifest(text, key):
    m = re.search(rf"^{key}\s*:=\s*(\S+)", text, re.M)
    return m.group(1) if m else None


def wider(root):
    bits = capability_bits(root)
    spawns = {p: p.read_text() for p in (root / "src").rglob("spawn.rs")}
    out, checked = [], 0
    for mk in sorted((root / "userland").glob("*/Capsule.mk")):
        text = mk.read_text()
        name = (manifest(text, "CAPSULE_SERVICE_ENDPOINT") or "").split(":")[-1]
        if not name:
            continue
        allowed = int(manifest(text, "CAPSULE_REQUIRED_CAPS") or "0", 16)
        allowed |= int(manifest(text, "CAPSULE_OPTIONAL_CAPS") or "0", 16)
        for path, src in spawns.items():
            if f'SERVICE_NAME: &str = "{name}"' not in src:
                continue
            consts = {m.group(1): value(m.group(2), bits, {})
                      for m in re.finditer(r"const (\w+): u64 =([^;]+);", src, re.S)}
            for m in re.finditer(r"requested_caps:\s*([^,]+?),\n", src, re.S):
                checked += 1
                grant = value(m.group(1), bits, consts)
                if grant & ~allowed:
                    out.append(f"{path.relative_to(root)}: grants {grant:#x} to {name}, whose manifest "
                               f"allows {allowed:#x} ({mk.relative_to(root)})")
    return out, checked


def stale_readmes(root):
    out = []
    for mk in sorted((root / "userland").glob("*/Capsule.mk")):
        required = manifest(mk.read_text(), "CAPSULE_REQUIRED_CAPS")
        readme = mk.parent / "README.md"
        if required is None or not readme.exists():
            continue
        for n, line in enumerate(readme.read_text().splitlines(), 1):
            for quoted in re.findall(r"CAPSULE_REQUIRED_CAPS\s*:?=\s*`?(0x[0-9a-fA-F]+)", line):
                if int(quoted, 16) != int(required, 16):
                    out.append(f"{readme.relative_to(root)}:{n}: quotes {quoted}, the manifest says {required}")
    return out


def self_test():
    with tempfile.TemporaryDirectory() as tmp:
        root = Path(tmp)
        (root / "src/capabilities/types").mkdir(parents=True)
        (root / DEFS).write_text("    IPC = 1 << 3,\n    Memory = 1 << 4,\n    Debug = 1 << 8,\n    Crypto = 1 << 5,\n")
        (root / "userland/capsule_decoy").mkdir(parents=True)
        (root / "userland/capsule_decoy/Capsule.mk").write_text(
            "CAPSULE_SERVICE_ENDPOINT := service:1:decoy\nCAPSULE_REQUIRED_CAPS := 0x18\n"
            "CAPSULE_OPTIONAL_CAPS := 0x100\n")
        spawn = root / "src/decoy/spawn.rs"
        spawn.parent.mkdir(parents=True)
        body = 'const SERVICE_NAME: &str = "decoy";\n    requested_caps: Capability::IPC.bit() | {},\n'
        spawn.write_text(body.format("crate::capabilities::serial_debug_cap()"))
        assert not wider(root)[0], "a grant within the manifest was reported"
        spawn.write_text(body.format("Capability::Crypto.bit()"))
        assert wider(root)[0], "a grant past the manifest was missed"
        spawn.write_text('const SERVICE_NAME: &str = "decoy";\nconst CAPS: u64 = 0x19;\n    requested_caps: CAPS,\n')
        assert wider(root)[0], "a constant past the manifest was missed"
        readme = root / "userland/capsule_decoy/README.md"
        readme.write_text("Mask: `CAPSULE_REQUIRED_CAPS = 0x18`.\n")
        assert not stale_readmes(root), "a README quoting its manifest was reported"
        readme.write_text("Mask: `CAPSULE_REQUIRED_CAPS = 0x19`.\n")
        assert stale_readmes(root), "a README quoting an old mask was missed"
    print("mirror caps: self-test passed, the decoys were reported")


def main():
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--root", type=Path, default=Path("."))
    ap.add_argument("--self-test", action="store_true")
    a = ap.parse_args()
    if a.self_test:
        self_test()
        return 0
    bad, checked = wider(a.root)
    for line in bad:
        print(f"mirror caps: {line}")
    stale = stale_readmes(a.root)
    for line in stale:
        print(f"mirror caps: {line}")
    print(f"mirror caps: {checked} grants checked, {len(bad)} wider than their manifest, "
          f"{len(stale)} READMEs quoting another mask")
    return 1 if bad or stale else 0


if __name__ == "__main__":
    sys.exit(main())
