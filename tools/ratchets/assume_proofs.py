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
"""The toolchains the proofs are checked with, and any escape hatch inside them."""

import re
import tomllib

EXTRACTORS = ["aeneas", "charon", "kani", "verus"]


def ours(path):
    """A file of the tree's own, not of a Lake package cache. `lake build`
    leaves every dependency's sources under .lake, mathlib's and Aeneas's
    axioms among them; none of them is something NØNOS rests on unless one
    of its own modules says so, and those are scanned."""
    return ".lake" not in path.parts


def tools(root):
    out = set()
    chan = tomllib.loads((root / "rust-toolchain.toml").read_text())["toolchain"]["channel"]
    out.add(f"tool:rustc-{chan}")
    lean = (root / "verification/lean/lean-toolchain").read_text().strip()
    out.add(f"tool:lean-{lean.rsplit(':', 1)[-1]}")
    ver = root / "verification"
    # The register is left out, or its own rows would confirm themselves.
    kinds = {".md", ".toml", ".py", ".lean", ".rs"}
    files = [p for p in ver.rglob("*")
             if p.is_file() and ours(p) and p.suffix in kinds and p.name != "ASSUMPTIONS.md"]
    text = "\n".join(p.read_text(errors="ignore").lower() for p in files)
    out |= {f"tool:{x}" for x in EXTRACTORS if re.search(rf"\b{x}\b", text)}
    return out


def proof_escapes(root):
    out = set()
    for p in (root / "verification").rglob("*.lean"):
        if not ours(p):
            continue
        for m in re.finditer(r"^\s*(?:private |protected )?axiom\s+([\w.']+)\s*[{(\[:]", p.read_text(errors="ignore"), re.M):
            out.add(f"lean-axiom:{m.group(1)}")
    for p in (root / "verification/verus").rglob("*.rs"):
        if re.search(r"external_body|\bassume\(", p.read_text(errors="ignore")):
            out.add(f"verus-assume:{p.relative_to(root)}")
    return out
