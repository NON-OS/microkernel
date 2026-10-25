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

//! A kernel file cut down to the code a glue check may match (L79). Line
//! and block comments go, nested or not, so text inside a comment never
//! passes for code; all whitespace goes, so a needle holds however the
//! formatter wraps a line. A string literal keeps its quotes, so a brace
//! inside it is never counted as code; in the bare form its text is
//! dropped, so a log line's words may change while the code around it may
//! not.

/// `text` with every comment and all whitespace cut, and each string
/// literal's text dropped too when `bare`.
pub(super) fn strip(text: &str, bare: bool) -> String {
    let mut out = String::new();
    let mut it = text.chars();
    let mut ahead = it.next();
    while let Some(c) = ahead {
        ahead = it.next();
        match (c, ahead) {
            ('/', Some('/')) => ahead = it.find(|&d| d == '\n'),
            ('/', Some('*')) => {
                let (mut depth, mut prev) = (1u32, ' ');
                while depth > 0 {
                    let d = it.next().expect("a block comment that never closes");
                    match (prev, d) {
                        ('/', '*') => (depth, prev) = (depth + 1, ' '),
                        ('*', '/') => (depth, prev) = (depth - 1, ' '),
                        _ => prev = d,
                    }
                }
                ahead = it.next();
            }
            ('"', _) => {
                out.push('"');
                let mut d = ahead;
                while let Some(ch) = d.filter(|&ch| ch != '"') {
                    let escaped = if ch == '\\' { it.next() } else { None };
                    if !bare && !ch.is_whitespace() {
                        out.push(ch);
                        out.extend(escaped);
                    }
                    d = it.next();
                }
                assert!(d.is_some(), "a string literal that never closes");
                out.push('"');
                ahead = it.next();
            }
            (c, _) if c.is_whitespace() => {}
            (c, _) => out.push(c),
        }
    }
    out
}
