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

/// Tokens of a multi-value CSS property, split only where the separator sits
/// outside every parenthesis and quote, so `28px clamp(24px, 5.2vw, 96px)`
/// is two words and `calc(2px + 1vw)` stays one.
pub(in crate::browser::css) struct SplitTop<'a> {
    rest: &'a str,
    comma: bool,
}

/// Whitespace-separated words at paren depth 0; empty words never appear.
pub(in crate::browser::css) fn words(value: &str) -> SplitTop<'_> {
    SplitTop { rest: value, comma: false }
}

/// Comma-separated items at paren depth 0, each trimmed. An empty item is
/// yielded as "" so a caller can reject a malformed list.
pub(in crate::browser::css) fn items(value: &str) -> SplitTop<'_> {
    SplitTop { rest: value, comma: true }
}

impl<'a> Iterator for SplitTop<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        if !self.comma {
            self.rest = self.rest.trim_start();
        }
        if self.rest.is_empty() {
            return None;
        }
        let (mut depth, mut quote) = (0i32, 0u8);
        let mut end = self.rest.len();
        for (i, b) in self.rest.bytes().enumerate() {
            match b {
                _ if quote != 0 => quote = if b == quote { 0 } else { quote },
                b'"' | b'\'' => quote = b,
                b'(' | b'[' => depth += 1,
                b')' | b']' => depth -= 1,
                b',' if self.comma && depth <= 0 => {
                    end = i;
                    break;
                }
                _ if !self.comma && depth <= 0 && b.is_ascii_whitespace() => {
                    end = i;
                    break;
                }
                _ => {}
            }
        }
        let token = &self.rest[..end];
        /* A comma is one byte; skip it so the next item starts after it. */
        self.rest = if self.comma && end < self.rest.len() {
            &self.rest[end + 1..]
        } else {
            &self.rest[end..]
        };
        Some(token.trim())
    }
}
