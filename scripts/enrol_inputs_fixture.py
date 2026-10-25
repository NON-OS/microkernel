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
"""Fixture crates for check_enrol_inputs --self-test: a tool and a library
that includes a trust file, the include gated or not, the gate's feature on
or off along the tool's edge."""

from pathlib import Path

INCLUDE = 'pub const R: [u8; 32] = *include_bytes!("../../nonos-data/trust/policy/r.bin");\n'


def write(base: Path, edge: str, gate: str, lib_features: str = "") -> Path:
    """The tool crate's path, its dependency `lib` written as `edge` and the
    module holding the include declared under `gate` (an attribute or "")."""
    tool, lib = base / "tool", base / "lib"
    for d in (tool / "src", lib / "src"):
        d.mkdir(parents=True, exist_ok=True)
    (tool / "Cargo.toml").write_text(f'[package]\nname = "tool"\n\n[dependencies]\nlib = {edge}\n')
    (tool / "src/main.rs").write_text("fn main() {}\n")
    (lib / "Cargo.toml").write_text(f'[package]\nname = "lib"\n\n[features]\n{lib_features}\n')
    (lib / "src/lib.rs").write_text(f"{gate}\nmod roots;\nmod check;\n")
    (lib / "src/roots.rs").write_text(INCLUDE)
    (lib / "src/check.rs").write_text("pub fn check() {}\n")
    return tool


ROOTS = 'default = ["roots"]\nroots = []\n'
OFF = '{ path = "../lib", default-features = false }'
ON = '{ path = "../lib" }'
GATED = '#[cfg(feature = "roots")]'

# (what, edge, gate, lib features, whether the tool compiles a trust file in)
CASES = [
    ("an ungated include", OFF, "", ROOTS, True),
    ("gated, the default left off", OFF, GATED, ROOTS, False),
    ("gated, the default on", ON, GATED, ROOTS, True),
    ("gated, the feature asked for", '{ path = "../lib", default-features = false, features = ["roots"] }',
     GATED, ROOTS, True),
    ("gated, a default feature turning it on", OFF.replace("false }", 'false, features = ["all"] }'),
     GATED, ROOTS + 'all = ["roots"]\n', True),
    ("a test module", ON, "#[cfg(test)]", "", False),
]
