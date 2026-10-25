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

"""What the tree says about a part: its ids in a driver table, its feature in the image."""

import pathlib
import re
import tomllib

ROOT = pathlib.Path(__file__).resolve().parents[2]
# The Standard build profile's kernel features (tools/nix/config.nix).
STANDARD = ["microkernel-full-gui"]


def image_features():
    """Every feature the Standard image's kernel turns on, closed over Cargo.toml."""
    table = tomllib.loads((ROOT / "Cargo.toml").read_text())["features"]
    seen, todo = set(), list(STANDARD)
    while todo:
        f = todo.pop()
        if f in seen or f not in table:
            continue
        seen.add(f)
        todo.extend(m for m in table[f] if "/" not in m)
    return seen


def table_text(rel):
    path = ROOT / rel
    if path.is_dir():
        return "\n".join(p.read_text(errors="replace") for p in sorted(path.rglob("*")) if p.is_file())
    return path.read_text(errors="replace") if path.is_file() else None


def found(token, text):
    """A PCI device id as the tables write it (0x15F2, 15f2), or an ACPI id."""
    kind, _, value = token.partition(":")
    if kind == "acpi":
        return value in text
    if kind in ("class", "usb") or value.endswith("*"):
        return None
    pattern = rf"(?<![0-9a-fA-Fx])(0x)?{value}(?![0-9a-fA-F])"
    return re.search(pattern, text, re.IGNORECASE) is not None


def in_table(ids, rel):
    """'k of n' named by the table, 'by class' for a class match, '-' for no ids."""
    if not ids:
        return "-"
    if all(i.split(":")[0] in ("class", "usb") for i in ids):
        return "by class"
    if rel == "-":
        return "no driver" if ids else "-"
    text = table_text(rel)
    if text is None:
        return f"table missing: {rel}"
    hits = [found(i, text) for i in ids]
    if not hits or all(h is None for h in hits):
        return "by class"
    named = [h for h in hits if h is not None]
    return f"{sum(named)} of {len(named)}"
