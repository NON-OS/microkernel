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
"""What docs/hardware/gpu*.md claims, read against the tree it describes.
`Tree` reads through an override map, so the self-test changes one fact at a
time without touching the tree."""

import re
from pathlib import Path
from check_display_profiles import enabled, features

PAGES = ["gpu.md", "gpu-intel.md", "gpu-amd.md", "gpu-nvidia.md"]
ROOTS = ("src/", "userland/", "scripts/", "nonos-bootloader/", "mk/", "docs/", "HANDOFF/")
IDS = "userland/capsule_driver_intel_display/data"
FAMILY = "userland/capsule_driver_intel_display/src/family.rs"
SUPPORT = "src/hardware/inventory/support.rs"
NATIVE = ["nonos-capsule-driver-intel-display", "nonos-capsule-driver-bga",
          "nonos-capsule-driver-bootfb"]


class Tree:
    def __init__(self, root, over=None, ids=None):
        self.root, self.over, self.ids_over = Path(root), over or {}, ids

    def read(self, rel):
        return self.over[rel] if rel in self.over else (self.root / rel).read_text()

    def exists(self, rel):
        if rel in self.over:
            return self.over[rel] is not None
        return any(self.root.glob(rel)) if "*" in rel else (self.root / rel).exists()

    def id_files(self):
        if self.ids_over is not None:
            return self.ids_over
        return sorted(p.name for p in (self.root / IDS).glob("gen*_ids.txt"))


def problems(t):
    out, pages = [], {p: t.read("docs/hardware/" + p) for p in PAGES}
    cargo = t.read("Cargo.toml")
    table, names = features(cargo), set(re.findall(r"^([a-z0-9-]+)\s*=\s*\[", cargo, re.M))
    for page, text in pages.items():
        for tok in re.findall(r"`([^`\s]+)`", text):
            if tok.startswith(ROOTS) and not t.exists(tok):
                out.append(f"{page}: `{tok}` does not exist")
            if tok.startswith(("nonos-capsule-", "microkernel-")) and tok not in names:
                out.append(f"{page}: `{tok}` is not a Cargo feature")
    intel = pages["gpu-intel.md"]
    out += [f"gpu-intel.md: {f} is not named" for f in t.id_files() if f"`{f}`" not in intel]
    named = set(re.findall(r"`(gen[0-9_]+_ids\.txt)`", intel))
    out += [f"gpu-intel.md: {f} names no id file" for f in named if f not in t.id_files()]
    image = enabled(table, "microkernel-full-gui")
    out += [f"gpu.md: the shipped image enables {n}" for n in NATIVE if n in image]
    if not re.search(r"fn sets_modes\(self\) -> bool \{\s*\*self\.gen\(\) == GEN9\s*\}",
                     t.read(FAMILY)):
        out.append("gpu-intel.md: sets_modes is no longer Gen9 alone")
    rows = [r for r in intel.splitlines() if r.startswith("|") and "| mode set |" in r]
    out += [f"gpu-intel.md: mode set row off Gen9: {r[:40]}" for r in rows if "| Gen9 |" not in r]
    if "DisplayNativeAmd | DisplayNativeNvidia => EnumerateOnly" not in t.read(SUPPORT):
        out.append("gpu-amd.md, gpu-nvidia.md: AMD or NVIDIA is no longer enumerate-only")
    return out
