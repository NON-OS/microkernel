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
"""The command line of boot_matrix.py."""

import argparse
from pathlib import Path


def esp_pair(text):
    name, _, path = text.partition("=")
    if not path:
        raise argparse.ArgumentTypeError("expected profile=path")
    return name, path


def parser(doc):
    ap = argparse.ArgumentParser(description=doc.splitlines()[0])
    ap.add_argument("--esp", type=esp_pair, action="append", default=[], help="profile=dir, once per profile")
    ap.add_argument("--ovmf")
    ap.add_argument("--ovmf-vars")
    ap.add_argument("--blk-img")
    ap.add_argument("--qemu", default="qemu-system-x86_64")
    ap.add_argument("--extra", default="", help="device arguments every cell gets")
    ap.add_argument("--accel", default="auto", choices=["auto", "kvm", "hvf", "tcg"])
    ap.add_argument("--repeat", type=int, default=5)
    ap.add_argument("--timeout", type=float, default=300.0, help="seconds per boot")
    ap.add_argument("--out", type=Path, default=Path("target/boot-matrix"))
    ap.add_argument("--list", action="store_true", help="name the cells and stop")
    ap.add_argument("cells", nargs="*")
    return ap


def arguments(doc, cells):
    """Parsed arguments, with the inputs a run needs checked for presence."""
    ap = parser(doc)
    a = ap.parse_args()
    if a.list:
        return a
    for name in ("ovmf", "ovmf_vars", "blk_img"):
        if getattr(a, name) is None:
            ap.error(f"--{name.replace('_', '-')} is required")
    unbuilt = sorted({c.profile for c in cells} - dict(a.esp).keys())
    if unbuilt:
        ap.error(f"no --esp given for profile {', '.join(unbuilt)}")
    return a
