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

//! Kernel glue read as text where it cannot run here (L79), cut down by
//! glue_lex to code alone: no comment, no whitespace, and in the bare form
//! no string literal's text.

use std::path::Path;

use super::glue_lex::strip;

fn read(rel: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../src").join(rel);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{rel}: {e}"))
}

/// The kernel file at `rel` under its src/, as code with no comment or
/// whitespace left in it.
pub(super) fn code(rel: &str) -> String {
    strip(&read(rel), false)
}

/// As `code`, with each string literal's text dropped too.
pub(super) fn bare(rel: &str) -> String {
    strip(&read(rel), true)
}

/// What lies between the braces of the item whose text up to and with its
/// opening brace is `head`; a brace inside a string literal is not counted.
pub(super) fn body<'a>(text: &'a str, head: &str) -> &'a str {
    let open = text.find(head).unwrap_or_else(|| panic!("missing `{head}`")) + head.len();
    let (mut depth, mut quoted, mut escaped) = (1usize, false, false);
    for (i, c) in text[open..].char_indices() {
        match (quoted, escaped, c) {
            (true, true, _) => escaped = false,
            (true, false, '\\') => escaped = true,
            (_, _, '"') => quoted = !quoted,
            (false, _, '{') => depth += 1,
            (false, _, '}') => depth -= 1,
            _ => {}
        }
        if depth == 0 {
            return &text[open..open + i];
        }
    }
    panic!("`{head}` never closes")
}

/// Where each needle starts, each found only past the end of the one
/// before it, so the call fails when one is missing or out of order.
pub(super) fn in_order(text: &str, needles: &[&str]) -> Vec<usize> {
    let mut from = 0;
    let mut starts = Vec::new();
    for n in needles {
        let at = text[from..].find(n).unwrap_or_else(|| panic!("`{n}` missing or out of order"));
        starts.push(from + at);
        from += at + n.len();
    }
    starts
}
