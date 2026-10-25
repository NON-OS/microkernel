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

//! A function's body found in the kernel's text, and the files under src/
//! that name a token, found by walking it.

use super::text::code;
use super::text_uses::every_file;

/// What lies between the braces of the item `head` opens, in `code`.
pub fn body<'a>(code: &'a str, head: &str) -> &'a str {
    let start = at(code, head);
    let open = start + at(&code[start..], "{");
    let mut depth = 0;
    for (i, c) in code[open..].char_indices() {
        depth += i32::from(c == '{') - i32::from(c == '}');
        if depth == 0 {
            return &code[open + 1..open + i];
        }
    }
    panic!("`{head}` never closes")
}

/// The flattened body of `head` in the kernel file `rel`.
pub fn flat_body(rel: &str, head: &str) -> String {
    flat(body(&code(rel), head))
}

/// `s` with every space and line break taken out.
pub fn flat(s: &str) -> String {
    s.split_whitespace().collect()
}

pub fn at(text: &str, needle: &str) -> usize {
    text.find(needle).unwrap_or_else(|| panic!("missing `{needle}`"))
}

/// Every .rs file under src/ whose code names `token`, as paths under src/.
pub fn naming(token: &str) -> Vec<String> {
    every_file().into_iter().filter(|(_, c)| c.contains(token)).map(|(rel, _)| rel).collect()
}
