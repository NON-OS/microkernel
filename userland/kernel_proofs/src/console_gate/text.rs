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

//! The kernel's text, read where the model cannot run it: found from this
//! crate's directory, its comments taken out so a step commented away is gone.

use std::path::{Path, PathBuf};

/// The kernel's src/, beside the userland this crate sits in.
pub fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../src")
}

/// The kernel file at `rel` under src/, its comments taken out.
pub fn code(rel: &str) -> String {
    let path = root().join(rel);
    let text = std::fs::read(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    strip(&String::from_utf8_lossy(&text))
}

/// `text` without its comments. A string is copied whole, so a comment
/// mark inside one is kept; a quote mark as a character opens no string.
pub fn strip(text: &str) -> String {
    let (mut out, mut rest) = (String::new(), text);
    while let Some(c) = rest.chars().next() {
        let (end, keep) = if rest.starts_with("//") {
            (rest.find('\n').unwrap_or(rest.len()), false)
        } else if rest.starts_with("/*") {
            (rest.find("*/").map_or(rest.len(), |i| i + 2), false)
        } else if rest.starts_with("'\"'") {
            (3, true)
        } else if c == '"' {
            (string_end(rest), true)
        } else {
            (c.len_utf8(), true)
        };
        if keep {
            out.push_str(&rest[..end]);
        }
        rest = &rest[end..];
    }
    out
}

/// Where the string `s` opens ends, past its closing quote.
fn string_end(s: &str) -> usize {
    let mut escaped = false;
    for (i, c) in s.char_indices().skip(1) {
        if c == '"' && !escaped {
            return i + 1;
        }
        escaped = c == '\\' && !escaped;
    }
    s.len()
}

/// How many braces are open at byte `i` of `text`.
pub fn depth(text: &str, i: usize) -> i32 {
    text[..i].chars().map(|c| i32::from(c == '{') - i32::from(c == '}')).sum()
}
