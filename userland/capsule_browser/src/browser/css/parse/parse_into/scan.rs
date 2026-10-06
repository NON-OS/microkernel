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

/* Byte index of the first of `stops` at nesting depth 0 outside a string,
 * else s.len(). (), [] and {} nest; a stop that opens a group is found
 * first. A string ends at its quote or a newline, as a bad string does in
 * CSS, so one stray quote cannot swallow the sheet. */
pub(in crate::browser::css::parse) fn find_top(s: &str, stops: &[u8]) -> usize {
    let b = s.as_bytes();
    let (mut depth, mut quote, mut i) = (0u32, 0u8, 0usize);
    while i < b.len() {
        let c = b[i];
        if c == b'\\' {
            i += 2;
            continue;
        }
        if quote != 0 {
            if c == quote || c == b'\n' {
                quote = 0;
            }
        } else if depth == 0 && stops.contains(&c) {
            return i;
        } else {
            match c {
                b'"' | b'\'' => quote = c,
                b'(' | b'[' | b'{' => depth += 1,
                b')' | b']' | b'}' => depth = depth.saturating_sub(1),
                _ => {}
            }
        }
        i += 1;
    }
    b.len()
}

/* The parts of `s` between top-level `sep`s, trimmed; empty ones kept. */
pub(in crate::browser::css::parse) fn split_top(s: &str, sep: u8) -> impl Iterator<Item = &str> {
    let mut rest = Some(s);
    core::iter::from_fn(move || {
        let r = rest?;
        let at = find_top(r, &[sep]);
        if at >= r.len() {
            rest = None;
            return Some(r.trim());
        }
        rest = r.get(at + 1..);
        Some(r[..at].trim())
    })
}

/* Where the item at the start of `rest` ends: at a ';' or a block. At top
 * level a qualified rule runs to its block; in a style rule's block a
 * custom property's value may hold braces and runs to its ';'. */
pub(in crate::browser::css::parse) fn item_end(rest: &str, nested: bool) -> usize {
    let stops: &[u8] = match (nested, rest.starts_with("--"), rest.starts_with('@')) {
        (true, true, _) => b";",
        (true, false, _) | (false, _, true) => b";{",
        (false, _, false) => b"{",
    };
    find_top(rest, stops)
}
