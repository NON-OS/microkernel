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

use super::super::super::node::Ns;
use super::super::ops::chars::split_space;
use super::super::ops::link::Loc;
use super::super::ops::mode::Mode;
use super::super::ops::state::Builder;

impl Builder {
    /// The "before html" mode: the html element, written or implied.
    pub(in super::super) fn before_html(&mut self, token: Token) {
        match token {
            Token::Doctype(_) | Token::Comment => {}
            Token::Chars(s) => {
                let (_, rest) = split_space(s);
                if !rest.is_empty() {
                    self.open_html(Tag::implied("html"));
                    self.reprocess(Mode::BeforeHead, Token::Chars(rest));
                }
            }
            Token::Start(t) if t.name == "html" => {
                self.open_html(t);
                self.mode = Mode::BeforeHead;
            }
            Token::End(t) if !matches!(&*t.name, "head" | "body" | "html" | "br") => {}
            other => {
                self.open_html(Tag::implied("html"));
                self.reprocess(Mode::BeforeHead, other);
            }
        }
    }

    /// The html element goes straight under the document.
    fn open_html(&mut self, tag: Tag) {
        if let Some(id) = self.create(tag, Ns::Html) {
            self.link(Loc::end(0), id);
            self.push_open(id);
        }
    }
}
