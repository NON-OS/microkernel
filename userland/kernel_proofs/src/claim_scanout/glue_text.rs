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

//! The kernel's glue read as text for the takeover's glue tests (L79): a
//! file found from this crate's own directory, its comments dropped and its
//! whitespace flattened to single spaces, and a function's body cut out by
//! its braces, so each check names exact code and the order of its calls.

use std::path::Path;

/// `rel` under the kernel's `src`, flattened.
pub(super) fn source(rel: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../src").join(rel);
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{rel}: {e}"));
    let code: Vec<&str> = text.lines().map(|l| l.split("//").next().unwrap_or("")).collect();
    code.join(" ").split_whitespace().collect::<Vec<_>>().join(" ")
}

/// The body of `fn name(` in `rel`, from its first `{` to the brace that
/// closes it.
pub(super) fn body(rel: &str, name: &str) -> String {
    let text = source(rel);
    let from = at(&text, &format!("fn {name}("));
    let open = from + at(&text[from..], "{");
    let mut depth = 0;
    for (i, c) in text[open..].char_indices() {
        depth += match c {
            '{' => 1,
            '}' => -1,
            _ => 0,
        };
        if depth == 0 {
            return text[open..=open + i].to_string();
        }
    }
    panic!("`fn {name}` in {rel} never closes");
}

pub(super) fn at(text: &str, needle: &str) -> usize {
    text.find(needle).unwrap_or_else(|| panic!("missing `{needle}`"))
}

/// Each of `needles` found, each after the one before it.
pub(super) fn in_order(text: &str, needles: &[&str]) {
    let mut from = 0;
    for n in needles {
        let found = text[from..].find(n);
        from += found.unwrap_or_else(|| panic!("`{n}` missing or out of order")) + n.len();
    }
}

/// How many times `needle` appears.
pub(super) fn count(text: &str, needle: &str) -> usize {
    text.matches(needle).count()
}
