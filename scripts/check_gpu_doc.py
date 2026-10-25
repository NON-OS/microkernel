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
"""The GPU pages in docs/hardware match the tree they describe: what they name
exists, every Intel id file is named, and their claims on the shipped image,
Gen9 mode setting and AMD and NVIDIA being enumerate-only hold in the code."""

import sys
import gate
from gpu_doc import FAMILY, SUPPORT, Tree, problems


def mutants(root):
    """One changed fact each, the check's name for it, and the change."""
    t = Tree(root)
    intel = "docs/hardware/gpu-intel.md"
    cargo = t.read("Cargo.toml")
    gui = cargo.replace('microkernel-full-gui = [\n',
                        'microkernel-full-gui = [\n  "nonos-capsule-driver-intel-display",\n', 1)
    return [
        ("a renamed crate", {intel: t.read(intel).replace(
            "userland/intel_display_proofs", "userland/intel_display_proof")}, None),
        ("an id file added", {}, t.id_files() + ["gen13_ids.txt"]),
        ("an id file removed", {}, [f for f in t.id_files() if f != "gen11_ids.txt"]),
        ("intel_display in the shipped image", {"Cargo.toml": gui}, None),
        ("Gen11 sets modes", {FAMILY: t.read(FAMILY).replace(
            "*self.gen() == GEN9", "*self.gen() == GEN9 || *self.gen() == GEN11")}, None),
        ("AMD given a data path", {SUPPORT: t.read(SUPPORT).replace(
            "DisplayNativeAmd | DisplayNativeNvidia => EnumerateOnly",
            "DisplayNativeNvidia => EnumerateOnly")}, None),
        ("a mode set row off Gen9", {intel: t.read(intel).replace(
            "| Ice Lake | Gen11 | `gen11_ids.txt` | takeover |",
            "| Ice Lake | Gen11 | `gen11_ids.txt` | mode set |")}, None),
    ]


def self_test(root):
    """Every mutant changes something and adds a problem the tree lacks."""
    base, missed = set(problems(Tree(root))), []
    for name, over, ids in mutants(root):
        changed = ids is not None or any(Tree(root).read(k) != v for k, v in over.items())
        if not changed or not set(problems(Tree(root, over, ids))) - base:
            missed.append(name)
    return missed


def main():
    args = gate.parser(__doc__).parse_args()
    if args.self_test:
        missed = self_test(args.root)
        print("gpu-doc self-test:", "bites" if not missed else "MISSED " + ", ".join(missed))
        return int(bool(missed))
    found = problems(Tree(args.root))
    for p in found:
        print("gpu-doc:", p)
    print("gpu-doc:", "ok" if not found else f"{len(found)} claims out of step")
    return int(bool(found))


if __name__ == "__main__":
    sys.exit(main())
