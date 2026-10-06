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

use super::super::ops::chars::only_space;
use super::super::ops::mode::Mode;
use super::super::ops::state::Builder;

impl Builder {
    /// The "in frameset" mode.
    pub(in super::super) fn in_frameset(&mut self, token: Token) {
        match token {
            Token::Start(t) if t.name == "frameset" => {
                self.insert_html(t);
            }
            Token::Start(t) if t.name == "frame" => {
                self.insert_void(t);
            }
            Token::End(t) if t.name == "frameset" && self.open.len() > 1 => {
                self.pop();
                if self.context.is_none() && !self.cur_is("frameset") {
                    self.mode = Mode::AfterFrameset;
                }
            }
            /* The rest is as after the frameset, where </html> alone differs. */
            other => self.after_frameset(other),
        }
    }

    /// The "after frameset" mode; "in frameset" and "after after frameset"
    /// share its rules for the tokens they do not handle themselves.
    pub(in super::super) fn after_frameset(&mut self, token: Token) {
        match token {
            Token::Chars(s) => self.frameset_text(&s),
            Token::Start(t) if t.name == "html" => self.in_body(Token::Start(t)),
            Token::Start(t) if t.name == "noframes" => self.in_head(Token::Start(t)),
            Token::End(t) if t.name == "html" && self.mode == Mode::AfterFrameset => {
                self.mode = Mode::AfterAfterFrameset;
            }
            Token::Eof => self.stopped = true,
            _ => {}
        }
    }

    /// The "after after frameset" mode.
    pub(in super::super) fn after_after_frameset(&mut self, token: Token) {
        match token {
            Token::Chars(s) => {
                let space = only_space(&s);
                if !space.is_empty() {
                    self.body_text(Cow::Owned(space));
                }
            }
            other => self.after_frameset(other),
        }
    }
}
