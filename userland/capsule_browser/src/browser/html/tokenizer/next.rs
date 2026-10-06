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

use super::state::{is_ws, TextMode, Tokenizer};
use super::token::Token;

impl<'a> Tokenizer<'a> {
    /// The next token. Each call reads exactly one: a run of text, a tag, a
    /// comment, a doctype, or the end of the input, so the tree builder can
    /// switch the state between any two of them.
    pub fn next_token(&mut self) -> Token<'a> {
        loop {
            if self.pos >= self.src.len() {
                return Token::Eof;
            }
            /*
             * None means input was consumed without producing a token, as
             * with "</>" or a tag cut off by the end of the input.
             */
            let token = match self.mode {
                TextMode::Data => self.data(),
                TextMode::Rcdata => self.raw(true),
                TextMode::Rawtext => self.raw(false),
                TextMode::ScriptData => self.script(),
                TextMode::Plaintext => Some(self.plaintext()),
            };
            if let Some(t) = token {
                return t;
            }
        }
    }

    /// The byte at the read position.
    pub(super) fn peek(&self) -> Option<u8> {
        self.src.as_bytes().get(self.pos).copied()
    }

    pub(super) fn skip_ws(&mut self) {
        while self.peek().is_some_and(is_ws) {
            self.pos += 1;
        }
    }
}
