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

use crate::browser::html::tokenizer::Token;

use super::super::ops::chars::{only_space, split_space};
use super::super::ops::mode::Mode;
use super::super::ops::state::Builder;

impl Builder {
    /// The "after body" mode. Content after </body> or </html> goes back
    /// into the body, which is where every browser renders it.
    pub(in super::super) fn after_body(&mut self, token: Token) {
        match token {
            Token::Chars(s) => {
                let (space, rest) = split_space(s);
                if !space.is_empty() {
                    self.body_text(space);
                }
                if !rest.is_empty() {
                    self.reprocess(Mode::InBody, Token::Chars(rest));
                }
            }
            Token::Comment | Token::Doctype(_) => {}
            Token::Start(t) if t.name == "html" => self.in_body(Token::Start(t)),
            Token::End(t) if t.name == "html" => {
                if self.context.is_none() {
                    self.mode = Mode::AfterAfterBody;
                }
            }
            Token::Eof => self.stopped = true,
            other => self.reprocess(Mode::InBody, other),
        }
    }

    /// The "after after body" mode.
    pub(in super::super) fn after_after_body(&mut self, token: Token) {
        match token {
            Token::Comment => {}
            Token::Doctype(_) => {}
            Token::Start(t) if t.name == "html" => self.in_body(Token::Start(t)),
            Token::Chars(s) => {
                let (space, rest) = split_space(s);
                if !space.is_empty() {
                    self.body_text(space);
                }
                if !rest.is_empty() {
                    self.reprocess(Mode::InBody, Token::Chars(rest));
                }
            }
            Token::Eof => self.stopped = true,
            other => self.reprocess(Mode::InBody, other),
        }
    }

    /// Whitespace kept in the frameset modes, which drop everything else.
    pub(in super::super) fn frameset_text(&mut self, s: &str) {
        self.insert_text(Cow::Owned(only_space(s)));
    }
}
