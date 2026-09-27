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
"""Where the tree says what it trusts: each detector returns assumption ids."""

import re
import tomllib

from assume_proofs import proof_escapes, tools

CRYPTO_FAMILIES = ["hash", "asymmetric", "pqc", "symmetric"]
CRYPTO_WHOLE = ["zk", "zk_kernel"]
# (id, pattern over kernel source text). Each names a hardware promise.
HARDWARE = [
    ("hw:rdrand", re.compile(r"\brd(rand|seed)\b", re.I)),
    ("hw:smep-smap", re.compile(r"\bSM[EA]P\b")),
    ("hw:nx", re.compile(r"\bNO_EXECUTE\b|\bNXE\b")),
    ("hw:iommu", re.compile(r"\bDMAR\b|\bVT-?d\b", re.I)),
    ("hw:tpm", re.compile(r"\bTPM2?_", re.I)),
]


def external_deps(manifest, prefix):
    d = tomllib.loads(manifest.read_text())
    tables = [d.get("dependencies", {})]
    tables += [t.get("dependencies", {}) for t in d.get("target", {}).values()]
    out = set()
    for table in tables:
        for name, spec in table.items():
            if not (isinstance(spec, dict) and "path" in spec):
                out.add(f"{prefix}:{name}")
    return out


def crypto_impls(root):
    base = root / "src" / "crypto"
    out = {f"prim:crypto/{w}" for w in CRYPTO_WHOLE if (base / w).exists()}
    for fam in CRYPTO_FAMILIES:
        for p in sorted((base / fam).iterdir()):
            if p.name != "mod.rs":
                out.add(f"prim:crypto/{fam}/{p.stem}")
    out.add("prim:stark-core")
    return out


def hardware(root):
    text = "\n".join(p.read_text(errors="ignore") for p in (root / "src").rglob("*.rs"))
    return {hid for hid, pat in HARDWARE if pat.search(text)}


def found(root):
    return (external_deps(root / "Cargo.toml", "crate")
            | external_deps(root / "nonos-bootloader/Cargo.toml", "boot-crate")
            | crypto_impls(root) | hardware(root) | tools(root) | proof_escapes(root))
