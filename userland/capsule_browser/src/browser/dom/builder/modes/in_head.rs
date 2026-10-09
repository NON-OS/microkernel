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

use crate::browser::html::tokenizer::{Tag, TextMode, Token};

use super::super::ops::chars::split_space;
use super::super::ops::mode::Mode;
use super::super::ops::state::Builder;

impl Builder {
    /// The "in head" mode.
    pub(in super::super) fn in_head(&mut self, token: Token) {
        match token {
            Token::Chars(s) => {
                let (space, rest) = split_space(s);
                self.insert_text(space);
                if !rest.is_empty() {
                    self.leave_head(Token::Chars(rest));
                }
            }
            Token::Comment | Token::Doctype(_) => {}
            Token::Start(t) => self.in_head_start(t),
            Token::End(t) => match &*t.name {
                "head" => {
                    self.pop();
                    self.mode = Mode::AfterHead;
                }
                "body" | "html" | "br" => self.leave_head(Token::End(t)),
                "template" => self.template_end(),
                _ => {}
            },
            other => self.leave_head(other),
        }
    }

    fn in_head_start(&mut self, t: Tag) {
        match &*t.name {
            "html" => self.in_body(Token::Start(t)),
            "base" | "basefont" | "bgsound" | "link" | "meta" => {
                self.insert_void(t);
            }
            "title" => self.raw_element(t, TextMode::Rcdata),
            "noframes" | "style" => self.raw_element(t, TextMode::Rawtext),
            /* Scripting is off for the parse, so noscript holds markup. */
            "noscript" => {
                self.insert_html(t);
                self.mode = Mode::InHeadNoscript;
            }
            "script" => self.raw_element(t, TextMode::ScriptData),
            "template" => self.template_start(t),
            "head" => {}
            _ => self.leave_head(Token::Start(t)),
        }
    }

    /// Anything else: the head is done, and the token goes on after it.
    fn leave_head(&mut self, token: Token) {
        self.pop();
        self.reprocess(Mode::AfterHead, token);
    }
}
