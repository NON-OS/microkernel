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

//! Kernel source read as code alone, for the glue checks that pin it whole.

/// The code alone: `//` and `/* */` comments cut, each where it starts, so
/// code commented out either way is gone; whitespace dropped.
pub(crate) fn code(text: &str) -> String {
    let (mut plain, mut rest) = (String::new(), text);
    while let Some(i) = rest.find(['/']) {
        plain.push_str(&rest[..i]);
        rest = &rest[i..];
        let close = if rest.starts_with("//") {
            "\n"
        } else if rest.starts_with("/*") {
            "*/"
        } else {
            ""
        };
        if close.is_empty() {
            plain.push('/');
            rest = &rest[1..];
            continue;
        }
        rest = rest[2..].find(close).map_or("", |j| &rest[2 + j + close.len()..]);
        plain.push('\n');
    }
    plain.push_str(rest);
    plain.split_whitespace().collect()
}

/// The code of `text` from `from` up to `to`.
pub(crate) fn between(text: &str, from: &str, to: &str) -> String {
    let (c, from, to) = (code(text), code(from), code(to));
    let a = c.find(&from).unwrap_or_else(|| panic!("missing `{from}`"));
    let b = c[a..].find(&to).unwrap_or_else(|| panic!("missing `{to}`"));
    c[a..a + b].to_string()
}

/// The code of `text` from `from` to its end.
pub(crate) fn tail(text: &str, from: &str) -> String {
    let c = code(text);
    c[c.find(&code(from)).unwrap_or_else(|| panic!("missing `{from}`"))..].to_string()
}
