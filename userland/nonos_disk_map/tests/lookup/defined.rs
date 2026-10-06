// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! A constant read out of another tree's source: the Rust `const NAME` or
//! the Python `NAME =` line, its value a sum of products of integers and of
//! names the same file defines.

use std::path::PathBuf;

/// The file at `rel` from the repository root.
pub fn source(rel: &str) -> String {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    std::fs::read_to_string(root.join(rel)).unwrap_or_else(|e| panic!("{rel}: {e}"))
}

/// The right-hand side of `name`'s definition, without the semicolon.
pub fn rhs(src: &str, name: &str) -> String {
    let rust_head = format!("{name}:");
    for line in src.lines() {
        let t = line.trim_start();
        let rust = t.split_once("const ").map(|(_, r)| r).filter(|r| r.starts_with(&rust_head));
        let python = t.strip_prefix(name).filter(|r| r.trim_start().starts_with('='));
        if let Some((_, v)) = rust.or(python).and_then(|def| def.split_once('=')) {
            return v.trim().trim_end_matches(';').trim().to_string();
        }
    }
    panic!("no definition of {name}")
}

/// The value of `name`: integers and names, `*` binding before `+`.
pub fn value(src: &str, name: &str) -> u64 {
    let text = rhs(src, name);
    text.split('+')
        .map(|term| term.split('*').map(|f| factor(src, f.trim())).product::<u64>())
        .sum()
}

fn factor(src: &str, f: &str) -> u64 {
    match f.replace('_', "").parse() {
        Ok(v) => v,
        Err(_) => value(src, f),
    }
}

/// How the sources spell a magic: `b"NONOSTR1"`, with or without a `*`.
pub fn magic(m: &[u8; 8]) -> String {
    format!("b\"{}\"", String::from_utf8_lossy(m))
}
