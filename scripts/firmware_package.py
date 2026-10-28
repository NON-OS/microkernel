#!/usr/bin/env python3
# NONOS Operating System
# Copyright (C) 2026 NONOS Contributors
# SPDX-License-Identifier: AGPL-3.0-or-later
"""Place the MediaTek firmware package on a boot medium: the six files the
mt7921e driver pins (userland/capsule_driver_mt7921/src/fw/pins.rs), each of
its pinned length and sha256, with linux-firmware's LICENCE.mediatek beside
them, under ESP/EFI/nonos/firmware/mediatek/. The files are never in the
tree and never in a capsule or the kernel; the loader reads them from here
at boot and the driver checks them again.

    firmware_package.py --esp DIR [--source DIR | --fetch CACHE] [--optional]

--source names a linux-firmware tree (or a copy holding mediatek/ and
LICENSES/); --fetch downloads the pinned commit's files into CACHE (kept
outside the tree, e.g. target/firmware). A file of any other length or hash
is refused and nothing of it is placed. With --optional, a package that
cannot be had is a warning and the medium is packed without it: the driver
then says "firmware package missing" rather than a build failing offline.
"""

import argparse
import hashlib
import shutil
import sys
import urllib.request
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from mediatek_firmware import ESP_DIR, LICENCE, UPSTREAM, commit, pins  # noqa: E402

ROOT = Path(__file__).resolve().parent.parent


def wanted(root):
    files = [(f"mediatek/{n}", length, sha) for n, length, sha in pins(root)]
    return files + [(LICENCE[0], None, LICENCE[1])]


def fetch(cache: Path, rev: str, rel: str) -> Path:
    out = cache / rel
    if not out.exists():
        out.parent.mkdir(parents=True, exist_ok=True)
        req = urllib.request.Request(f"{UPSTREAM}{rel}?id={rev}",
                                     headers={"User-Agent": "nonos-firmware-package"})
        with urllib.request.urlopen(req, timeout=120) as r:
            out.write_bytes(r.read())
    return out


def check(path: Path, length, sha) -> str:
    data = path.read_bytes()
    if length is not None and len(data) != length:
        return f"{path.name}: {len(data)} bytes, pinned {length}"
    if hashlib.sha256(data).hexdigest() != sha:
        return f"{path.name}: sha256 does not match its pin"
    return ""


def gather(a, rev):
    found = []
    for rel, length, sha in wanted(ROOT):
        try:
            path = fetch(Path(a.fetch), rev, rel) if a.fetch else Path(a.source) / rel
        except OSError as e:
            return None, f"{rel}: could not be fetched ({e})"
        if not path.is_file():
            return None, f"{rel}: not in {a.source}"
        bad = check(path, length, sha)
        if bad:
            if a.fetch:
                path.unlink()  # a cut or altered download is fetched afresh next time
            return None, bad
        found.append((rel, path))
    return found, ""


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--esp", required=True, help="the ESP directory (holds EFI/nonos)")
    src = ap.add_mutually_exclusive_group(required=True)
    src.add_argument("--source")
    src.add_argument("--fetch", metavar="CACHE")
    ap.add_argument("--optional", action="store_true")
    a = ap.parse_args()
    rev = commit(ROOT)
    found, why = gather(a, rev)
    dest = Path(a.esp) / "EFI" / "nonos" / ESP_DIR
    if found is None:
        print(f"firmware package: {why}", file=sys.stderr)
        if not a.optional:
            return 1
        print("firmware package: WARNING packed without it; mt7921e will report it missing")
        return 0
    if dest.exists():
        shutil.rmtree(dest)
    dest.mkdir(parents=True)
    for rel, path in found:
        shutil.copyfile(path, dest / Path(rel).name)
    print(f"firmware package: {len(found) - 1} MediaTek files and LICENCE.mediatek "
          f"(linux-firmware {rev[:12]}) at {dest}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
