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

use super::state::Tokenizer;
use super::token::Token;

impl<'a> Tokenizer<'a> {
    /// Markup at a "<" that `markup_starts` accepted: the tag open state.
    pub(super) fn markup(&mut self) -> Option<Token<'a>> {
        let next = self.src.as_bytes()[self.pos + 1];
        self.pos += if matches!(next, b'!' | b'/') { 2 } else { 1 };
        match next {
            b'!' => self.declaration(),
            b'/' => self.end_tag_open(),
            /* "<?" is a bogus comment that keeps its "?", ending at the first ">". */
            b'?' => Some(self.bogus_comment()),
            _ => self.tag(false),
        }
    }

    /// After "</": "</>" is dropped, "</" at the end is text, and anything
    /// other than a letter opens a bogus comment.
    fn end_tag_open(&mut self) -> Option<Token<'a>> {
        match self.peek() {
            None => Some(Token::Chars(Cow::Borrowed("</"))),
            Some(b'>') => {
                self.pos += 1;
                None
            }
            Some(c) if c.is_ascii_alphabetic() => self.tag(true),
            Some(_) => Some(self.bogus_comment()),
        }
    }

    /// After "<!": a comment, a doctype, a CDATA section where foreign
    /// content allows one, or else a bogus comment.
    fn declaration(&mut self) -> Option<Token<'a>> {
        let rest = &self.src.as_bytes()[self.pos..];
        if rest.starts_with(b"--") {
            self.pos += 2;
            return Some(self.comment());
        }
        if rest.len() >= 7 && rest[..7].eq_ignore_ascii_case(b"doctype") {
            self.pos += 7;
            return Some(self.doctype());
        }
        if self.foreign && rest.starts_with(b"[CDATA[") {
            self.pos += 7;
            return self.cdata();
        }
        Some(self.bogus_comment())
    }

    /// Everything to the next ">" (or the end) is a comment.
    pub(super) fn bogus_comment(&mut self) -> Token<'a> {
        self.skip_past_gt();
        Token::Comment
    }
}
