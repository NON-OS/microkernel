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
"""Put a Qwen tier on a NONOS disk: fetch it, check it, lay it out.

    nonos-qwen-tier.py list
    nonos-qwen-tier.py fetch TIER... --dir DIR
    nonos-qwen-tier.py plan TIER... --dir DIR --image IMAGE [--fresh]

The tiers, their files, lengths and SHA-256 digests are read from the
signed personality's table (pinned.rs), so this tool and the kernel can
never disagree on what a tier is. fetch runs on any machine with a network
and resumes a cut download; plan checks every file against its pin before
writing a byte, sizes the data volume to hold all the tiers asked for, and
hands the layout to nonos-data-plan.py. NONOS itself never downloads a
model: the disk carries the files, and the first boot seals and verifies
them into the encrypted volume.
"""

import argparse
import hashlib
import os
import re
import subprocess
import sys
import urllib.request

HERE = os.path.dirname(os.path.abspath(__file__))
PINS = os.path.join(HERE, "../userland/capsule_linux/src/linux/file/models/pinned.rs")
PIN = re.compile(r'tier: "(\w+)",\s*name: b"/([^"]+)",\s*bytes: ([\d_]+),\s*'
                 r'sha256: hex32\(b"([0-9a-f]{64})"\)')
HF = "https://huggingface.co/Qwen/Qwen2.5-{}-Instruct-GGUF/resolve/main/{}"
# 484 plain bytes per sealed 512-byte sector, one pointer block per 60, a margin.
PLAIN, FANOUT, SPARE = 484, 60, 65_536


def pins():
    out = {}
    for tier, name, size, digest in PIN.findall(open(PINS).read()):
        out.setdefault(tier, []).append((name, int(size.replace("_", "")), digest))
    return out


def check(path, size, digest):
    if not os.path.exists(path) or os.path.getsize(path) != size:
        return False
    h = hashlib.sha256()
    with open(path, "rb") as f:
        while chunk := f.read(1 << 24):
            h.update(chunk)
    return h.hexdigest() == digest


def fetch(name, size, digest, into):
    path, part = os.path.join(into, name), os.path.join(into, name + ".part")
    if check(path, size, digest):
        return print(f"{name}: present and verified")
    have = os.path.getsize(part) if os.path.exists(part) else 0
    req = urllib.request.Request(HF.format(name.split("-")[1].upper(), name))
    if have:
        req.add_header("Range", f"bytes={have}-")
    with urllib.request.urlopen(req) as r:
        # A server that ignores the range sends the whole file again.
        have = have if r.status == 206 else 0
        with open(part, "ab" if have else "wb") as f:
            copy(r, f, name, size)
    print()
    if not check(part, size, digest):
        os.remove(part)
        sys.exit(f"{name}: length or SHA-256 differs from the pin; removed")
    os.replace(part, path)
    print(f"{name}: verified {digest}")


def copy(r, f, name, size):
    while chunk := r.read(1 << 22):
        f.write(chunk)
        print(f"\r{name}: {f.tell() >> 20} of {size >> 20} MiB", end="", flush=True)


def volume_sectors(sizes):
    data = sum(-(-s // PLAIN) for s in sizes)
    return data + data // (FANOUT - 1) + SPARE


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("verb", choices=["list", "fetch", "plan"])
    ap.add_argument("tiers", nargs="*")
    ap.add_argument("--dir", default=".")
    ap.add_argument("--image")
    ap.add_argument("--fresh", action="store_true")
    a, table = ap.parse_args(), pins()
    if a.verb == "list":
        for tier, files in table.items():
            print(f"{tier:7} {sum(s for _, s, _ in files) / 1e9:5.2f} GB  "
                  + " ".join(n for n, _, _ in files))
        return
    unknown = [t for t in a.tiers if t not in table]
    if not a.tiers or unknown:
        ap.error(f"name one or more tiers of: {' '.join(table)}")
    files = [f for t in dict.fromkeys(a.tiers) for f in table[t]]
    if a.verb == "fetch":
        os.makedirs(a.dir, exist_ok=True)
        for name, size, digest in files:
            fetch(name, size, digest, a.dir)
        return
    if not a.image:
        ap.error("plan needs --image (a raw image file or a whole disk)")
    for name, size, digest in files:
        if not check(os.path.join(a.dir, name), size, digest):
            sys.exit(f"{name}: missing, or its length or SHA-256 differs from the pin")
    cmd = [sys.executable, os.path.join(HERE, "nonos-data-plan.py"), a.image,
           "--volume-sectors", str(volume_sectors([s for _, s, _ in files]))]
    cmd += [x for n, _, _ in files for x in ("--import", os.path.join(a.dir, n))]
    sys.exit(subprocess.call(cmd + (["--fresh"] if a.fresh else [])))


if __name__ == "__main__":
    main()
