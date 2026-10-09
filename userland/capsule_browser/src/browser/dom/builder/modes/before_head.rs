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
    /// The "before head" mode: the head element, written or implied.
    pub(in super::super) fn before_head(&mut self, token: Token) {
        match token {
            Token::Chars(s) => {
                let (_, rest) = split_space(s);
                if !rest.is_empty() {
                    self.open_head(Tag::implied("head"));
                    self.reprocess(Mode::InHead, Token::Chars(rest));
                }
            }
            Token::Comment | Token::Doctype(_) => {}
            Token::Start(t) if t.name == "html" => self.in_body(Token::Start(t)),
            Token::Start(t) if t.name == "head" => {
                self.open_head(t);
                self.mode = Mode::InHead;
            }
            Token::End(t) if !matches!(&*t.name, "head" | "body" | "html" | "br") => {}
            other => {
                self.open_head(Tag::implied("head"));
                self.reprocess(Mode::InHead, other);
            }
        }
    }

    fn open_head(&mut self, tag: Tag) {
        self.head = self.insert_html(tag);
    }
}
