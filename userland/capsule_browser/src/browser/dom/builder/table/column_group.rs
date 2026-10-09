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

use crate::browser::html::tokenizer::Token;

use super::super::ops::chars::split_space;
use super::super::ops::mode::Mode;
use super::super::ops::state::Builder;

impl Builder {
    /// The "in column group" mode.
    pub(in super::super) fn in_column_group(&mut self, token: Token) {
        match token {
            Token::Chars(s) => {
                let (space, rest) = split_space(s);
                self.insert_text(space);
                if !rest.is_empty() {
                    self.leave_colgroup(Token::Chars(rest));
                }
            }
            Token::Comment | Token::Doctype(_) => {}
            Token::Start(t) if t.name == "html" => self.in_body(Token::Start(t)),
            Token::Start(t) if t.name == "col" => {
                self.insert_void(t);
            }
            Token::End(t) if t.name == "colgroup" => {
                if self.cur_is("colgroup") {
                    self.pop();
                    self.mode = Mode::InTable;
                }
            }
            Token::End(t) if t.name == "col" => {}
            Token::Start(t) if t.name == "template" => self.in_head(Token::Start(t)),
            Token::End(t) if t.name == "template" => self.in_head(Token::End(t)),
            Token::Eof => self.in_body(Token::Eof),
            other => self.leave_colgroup(other),
        }
    }

    fn leave_colgroup(&mut self, token: Token) {
        if self.cur_is("colgroup") {
            self.pop();
            self.reprocess(Mode::InTable, token);
        }
    }
}
