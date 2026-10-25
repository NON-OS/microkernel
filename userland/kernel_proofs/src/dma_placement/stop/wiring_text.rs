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

//! Kernel source as wiring_tests reads it: one function's body, its `//`
//! comments dropped and its whitespace taken out.

/// The body of `fn name(` in `src`.
pub fn body(src: &str, name: &str) -> String {
    let code: Vec<&str> = src.lines().map(|l| l.split("//").next().unwrap_or("")).collect();
    let flat: String = code.concat().split_whitespace().collect();
    let at = flat.find(&format!("fn{name}(")).unwrap_or_else(|| panic!("no fn {name}"));
    let (mut depth, open) = (0i32, at + flat[at..].find('{').expect("a body"));
    let end = flat[open..].char_indices().find(|&(_, c)| {
        depth += i32::from(c == '{') - i32::from(c == '}');
        depth == 0
    });
    flat[open..open + end.expect("closed").0].to_string()
}

/// Whether `first` appears in `text`, and `then` after it.
pub fn before(text: &str, first: &str, then: &str) -> bool {
    matches!((text.find(first), text.find(then)), (Some(a), Some(b)) if a < b)
}
