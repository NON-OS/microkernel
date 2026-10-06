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

/* The media list after the location: `layer`, `layer(name)` and
 * `supports(...)` lead it when present. */
pub(super) fn media_part(rest: &str) -> &str {
    let mut r = rest.trim_start();
    loop {
        let end = r.find(|c: char| c == '(' || c.is_ascii_whitespace()).unwrap_or(r.len());
        let word = &r[..end];
        let call = r[end..].starts_with('(');
        if !(word.eq_ignore_ascii_case("layer") || (call && word.eq_ignore_ascii_case("supports")))
        {
            return r.trim();
        }
        r = r[if call { close_paren(r, end) } else { end }..].trim_start();
    }
}

/* The byte after the parenthesis closing the one opened at `open`. */
fn close_paren(s: &str, open: usize) -> usize {
    let mut depth = 0u32;
    for (i, b) in s.bytes().enumerate().skip(open) {
        match b {
            b'(' => depth += 1,
            b')' if depth <= 1 => return i + 1,
            b')' => depth -= 1,
            _ => {}
        }
    }
    s.len()
}
