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

use alloc::string::String;

/* Each top-level layer's gradient function, without the size, position
 * and repeat words a mask shorthand may carry beside it. */
pub(super) fn gradient_layers(v: &str) -> Option<String> {
    let mut out = String::new();
    for layer in top_level(v) {
        let open = layer.find("gradient(")? + "gradient".len();
        let name = layer[..open].rfind(|c: char| !(c.is_ascii_alphanumeric() || c == '-'));
        let close = close_paren(layer, open)?;
        if !out.is_empty() {
            out.push_str(", ");
        }
        out.push_str(&layer[name.map_or(0, |i| i + 1)..=close]);
    }
    (!out.is_empty()).then_some(out)
}

/* The comma-separated layers of `v`, commas inside parentheses kept. */
fn top_level(v: &str) -> impl Iterator<Item = &str> {
    let mut depth = 0i32;
    v.split(move |c| {
        depth += match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };
        c == ',' && depth == 0
    })
}

/* The index of the ')' closing the '(' at `open`. */
fn close_paren(s: &str, open: usize) -> Option<usize> {
    let mut depth = 0i32;
    for (i, c) in s[open..].char_indices() {
        depth += match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };
        if depth == 0 {
            return Some(open + i);
        }
    }
    None
}
