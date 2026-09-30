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
"""Fetch a pinned file from the Qwen team's Hugging Face repository, or
from another source serving the same bytes."""

import os
import re
import sys
import urllib.request

from .pins import check

HF = "https://huggingface.co/Qwen/{}/resolve/main/{}"
QWEN25 = re.compile(r"qwen2\.5-(coder-)?(\d+(?:\.\d+)?)b-instruct-q[\w-]+\.gguf")
QWEN3 = re.compile(r"Qwen3-(\d+(?:\.\d+)?B(?:-A\d+B)?)-Q[\w-]+\.gguf")


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


def fetch(name, size, digest, into, source=None):
    """`name` into the directory `into`, from `source` or its upstream URL."""
    os.makedirs(into, exist_ok=True)
    path, part = os.path.join(into, name), os.path.join(into, name + ".part")
    if check(path, size, digest):
        return print(f"{name}: present and verified")
    have = os.path.getsize(part) if os.path.exists(part) else 0
    if have < size:
        req = urllib.request.Request(source or url(name))
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
