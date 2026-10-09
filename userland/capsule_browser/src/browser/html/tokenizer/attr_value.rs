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

use crate::browser::html::entity::charref;

use super::limits::MAX_VALUE;
use super::state::{is_ws, Tokenizer};

impl<'a> Tokenizer<'a> {
    /// An attribute value after its "=" (13.2.5.35 to 13.2.5.38), quoted or
    /// up to whitespace or ">", with character references decoded in attribute
    /// mode. None when the input ends inside it; false when over `MAX_VALUE`.
    pub(super) fn attr_value(&mut self, value: &mut String) -> Option<bool> {
        if self.peek() == Some(b'>') {
            return Some(true);
        }
        let quote = self.peek().filter(|q| matches!(q, b'"' | b'\''));
        self.pos += usize::from(quote.is_some());
        let b = self.src.as_bytes();
        let ends = |c: u8| match quote {
            Some(q) => c == q,
            None => is_ws(c) || c == b'>',
        };
        let mut whole = true;
        loop {
            let from = self.pos;
            let stop = b[from..]
                .iter()
                .position(|&c| ends(c) || c == b'&' || c == b'\0')
                .map_or(b.len(), |n| from + n);
            whole = whole && value.len() + (stop - from) <= MAX_VALUE;
            if whole {
                value.push_str(&self.src[from..stop]);
            }
            self.pos = stop;
            match b.get(stop).copied() {
                None => return None,
                Some(b'&') => {
                    self.pos += 1;
                    let rest = &self.src[self.pos..];
                    let mut scratch = String::new();
                    self.pos += charref(rest, true, if whole { &mut *value } else { &mut scratch });
                }
                Some(b'\0') => {
                    self.pos += 1;
                    if whole {
                        value.push('\u{FFFD}');
                    }
                }
                Some(_) => break,
            }
        }
        self.pos += usize::from(quote.is_some());
        if !whole {
            *value = String::new();
        }
        Some(whole)
    }
}
