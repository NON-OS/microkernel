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
"""Fixture sources for check_embed_asserts --self-test: an embedding module
with its assert, without it, with one const left out of it, a module that
embeds no capsule, and the embedding macro with and without its assert."""

from pathlib import Path

INCLUDES = (
    '#[cfg(feature = "nonos-capsule-x")]\n'
    'pub(super) const X_ELF: &[u8] = include_bytes!(concat!("../../../userland/capsule_x/target/", "x"));\n'
    '#[cfg(feature = "nonos-capsule-x")]\n'
    'pub(super) const X_CERT_BYTES: &[u8] = include_bytes!("../../../nonos-data/trust/capsules/x.cert");\n'
)
ASSERTED = '#[cfg(feature = "nonos-capsule-x")]\nconst _: () = assert!(!X_ELF.is_empty() && !X_CERT_BYTES.is_empty(), "x");\n'
HALF = '#[cfg(feature = "nonos-capsule-x")]\nconst _: () = assert!(!X_ELF.is_empty(), "x");\n'
MACRO = 'macro_rules! tool { ($elf:expr) => { { const ELF: &[u8] = include_bytes!($elf);\n'
MACRO_ASSERT = 'const _: () = assert!(!ELF.is_empty(), "t");\n'


def write(base: Path, files: dict) -> Path:
    """A src/ under `base` holding `files`, path to text."""
    for rel, text in files.items():
        (base / rel).parent.mkdir(parents=True, exist_ok=True)
        (base / rel).write_text(text)
    return base


# (what, files, how many problems the scan must report)
CASES = [
    ("asserted", {"a/embed.rs": INCLUDES + ASSERTED}, 0),
    ("no assert", {"a/embed.rs": INCLUDES}, 1),
    ("a const left out", {"a/embed.rs": INCLUDES + HALF}, 1),
    ("no capsule embedded", {"a/blob.rs": 'const B: &[u8] = include_bytes!("x.bin");\n'}, 0),
    ("the macro asserted", {"t/m.rs": MACRO + MACRO_ASSERT + "} } }\n"}, 0),
    ("the macro without", {"t/m.rs": MACRO + "} } }\n"}, 1),
]
