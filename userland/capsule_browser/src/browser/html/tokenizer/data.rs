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

use alloc::borrow::Cow;

use crate::browser::html::entity::charref;

use super::run::{may_start_ref, replace_nul, Run};
use super::state::Tokenizer;
use super::token::Token;

impl<'a> Tokenizer<'a> {
    /// The data state: text with its character references decoded, up to the
    /// next piece of markup or NUL. A "<" that cannot start markup ("a < b",
    /// "x<5") is text, as is one at the very end.
    pub(super) fn data(&mut self) -> Option<Token<'a>> {
        let src: &'a str = self.src;
        let b = src.as_bytes();
        match b[self.pos] {
            b'<' if self.markup_starts() => return self.markup(),
            b'\0' => {
                self.pos += 1;
                return Some(Token::Null);
            }
            _ => {}
        }
        let mut run = Run::new(self.pos);
        loop {
            let from = self.pos;
            let n = b[from..].iter().position(|&c| matches!(c, b'<' | b'&' | b'\0'));
            let stop = n.map_or(b.len(), |n| from + n);
            self.pos = stop;
            match b.get(stop) {
                Some(b'<') if !self.markup_starts() => self.pos += 1,
                Some(b'&') if may_start_ref(b.get(stop + 1)) => {
                    let rest = &src[stop + 1..];
                    self.pos += run.decode(src, stop, |out| 1 + charref(rest, false, out));
                }
                Some(b'&') => self.pos += 1,
                _ => break,
            }
        }
        Some(Token::Chars(run.finish(src, self.pos)))
    }

    /// Whether the "<" at the current position opens markup: a letter, "/",
    /// "!" or "?" has to follow it.
    pub(super) fn markup_starts(&self) -> bool {
        let next = self.src.as_bytes().get(self.pos + 1).copied();
        next.is_some_and(|c| c.is_ascii_alphabetic() || matches!(c, b'/' | b'!' | b'?'))
    }

    /// PLAINTEXT: everything left is text, and nothing ends it.
    pub(super) fn plaintext(&mut self) -> Token<'a> {
        let src: &'a str = self.src;
        let text: Cow<'a, str> = replace_nul(&src[self.pos..]);
        self.pos = src.len();
        Token::Chars(text)
    }
}
