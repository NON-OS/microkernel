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

use crate::browser::html::tokenizer::{Tag, Token};

use super::super::ops::chars::split_space;
use super::super::ops::mode::Mode;
use super::super::ops::state::Builder;

impl Builder {
    /// The "after head" mode: the body element, written or implied.
    pub(in super::super) fn after_head(&mut self, token: Token) {
        match token {
            Token::Chars(s) => {
                let (space, rest) = split_space(s);
                self.insert_text(space);
                if !rest.is_empty() {
                    self.implied_body(Token::Chars(rest));
                }
            }
            Token::Comment | Token::Doctype(_) => {}
            Token::Start(t) => match &*t.name {
                "html" => self.in_body(Token::Start(t)),
                "body" => {
                    self.insert_html(t);
                    self.frameset_ok = false;
                    self.mode = Mode::InBody;
                }
                "frameset" => {
                    self.insert_html(t);
                    self.mode = Mode::InFrameset;
                }
                "base" | "basefont" | "bgsound" | "link" | "meta" | "noframes" | "script"
                | "style" | "template" | "title" => self.back_into_head(Token::Start(t)),
                "head" => {}
                _ => self.implied_body(Token::Start(t)),
            },
            Token::End(t) => match &*t.name {
                "template" => self.in_head(Token::End(t)),
                "body" | "html" | "br" => self.implied_body(Token::End(t)),
                _ => {}
            },
            other => self.implied_body(other),
        }
    }

    /// Head content that came after </head> still goes into the head.
    fn back_into_head(&mut self, token: Token) {
        let Some(head) = self.head else {
            return;
        };
        self.push_open(head);
        self.in_head(token);
        self.remove_open(head);
    }

    fn implied_body(&mut self, token: Token) {
        self.insert_html(Tag::implied("body"));
        self.frameset_ok = true;
        self.reprocess(Mode::InBody, token);
    }
}
