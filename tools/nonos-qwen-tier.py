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
signed personality's tables (pinned.rs and the pinned_*.rs beside it, one
a family), so this tool and the kernel can never disagree on what a tier
is. Each file comes from the Qwen team's Hugging Face repository for its
model: Qwen/Qwen2.5-<size>-Instruct-GGUF, Qwen/Qwen2.5-Coder-<size>-
Instruct-GGUF or Qwen/Qwen3-<size>-GGUF. fetch runs on any machine with a
network and resumes a cut download; plan checks every file against its pin
before writing a byte, sizes the data volume to hold all the tiers asked
for, and hands the layout to nonos-data-plan.py. NONOS itself never downloads a
model: the disk carries the files, and the first boot seals and verifies
them into the encrypted volume.
"""

import argparse
import glob
import hashlib
import os
import re
import subprocess
import sys
import urllib.request

HERE = os.path.dirname(os.path.abspath(__file__))
MODELS = os.path.join(HERE, "../userland/capsule_linux/src/linux/file/models")
PIN = re.compile(r'tier: "([\w.-]+)",\s*name: b"/([^"]+)",\s*bytes: ([\d_]+),\s*'
                 r'sha256: hex32\(b"([0-9a-f]{64})"\)')
TABLE = re.compile(r'pub const (\w+): &\[Pinned\] = &\[(.*?)\n\];', re.S)
FAMILIES = re.compile(r'FAMILIES: &\[&\[Pinned\]\] = &\[([\w, ]+)\]')
HF = "https://huggingface.co/Qwen/{}/resolve/main/{}"
QWEN25 = re.compile(r"qwen2\.5-(coder-)?(\d+(?:\.\d+)?)b-instruct-q[\w-]+\.gguf")
QWEN3 = re.compile(r"Qwen3-(\d+(?:\.\d+)?B(?:-A\d+B)?)-Q[\w-]+\.gguf")
# 484 plain bytes per sealed 512-byte sector, one pointer block per 60, a margin.
PLAIN, FANOUT, SPARE = 484, 60, 65_536


def pins():
    """Every tier's files, family by family in the order pinned.rs gives."""
    tables = {}
    for path in sorted(glob.glob(os.path.join(MODELS, "pinned*.rs"))):
        for const, body in TABLE.findall(open(path).read()):
            tables[const] = PIN.findall(body)
    order = FAMILIES.search(open(os.path.join(MODELS, "pinned.rs")).read())
    if not order or sorted(tables) != sorted(order.group(1).replace(" ", "").split(",")):
        sys.exit(f"{MODELS}: the tables and pinned.rs's FAMILIES do not agree")
    out = {}
    for const in order.group(1).replace(" ", "").split(","):
        for tier, name, size, digest in tables[const]:
            out.setdefault(tier, []).append((name, int(size.replace("_", "")), digest))
    return out


def repo(name):
    """The Hugging Face repository the Qwen team publishes `name` in."""
    if m := QWEN25.fullmatch(name):
        coder = "Coder-" if m.group(1) else ""
        return f"Qwen2.5-{coder}{m.group(2)}B-Instruct-GGUF"
    if m := QWEN3.fullmatch(name):
        return f"Qwen3-{m.group(1)}-GGUF"
    sys.exit(f"{name}: no known Qwen repository publishes this file")


def url(name):
    return HF.format(repo(name), name)


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
    req = urllib.request.Request(url(name))
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
            print(f"{tier:13} {sum(s for _, s, _ in files) / 1e9:5.2f} GB  "
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
