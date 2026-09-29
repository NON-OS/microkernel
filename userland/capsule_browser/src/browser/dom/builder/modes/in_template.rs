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

use super::super::ops::mode::Mode;
use super::super::ops::state::Builder;

impl Builder {
    /// The "in template" mode: the first tag inside a template decides which
    /// mode its contents are parsed in.
    pub(in super::super) fn in_template(&mut self, token: Token) {
        let next = match &token {
            Token::Chars(_) | Token::Null | Token::Comment | Token::Doctype(_) => {
                self.in_body(token);
                return;
            }
            Token::Start(t) => match &*t.name {
                "base" | "basefont" | "bgsound" | "link" | "meta" | "noframes" | "script"
                | "style" | "template" | "title" => {
                    self.in_head(token);
                    return;
                }
                "caption" | "colgroup" | "tbody" | "tfoot" | "thead" => Mode::InTable,
                "col" => Mode::InColumnGroup,
                "tr" => Mode::InTableBody,
                "td" | "th" => Mode::InRow,
                _ => Mode::InBody,
            },
            Token::End(t) if t.name == "template" => {
                self.in_head(token);
                return;
            }
            Token::End(_) => return,
            Token::Eof => {
                self.template_eof();
                return;
            }
        };
        self.tmpl.pop();
        self.tmpl.push(next);
        self.reprocess(next, token);
    }

    /// The input ended inside a template: close it and try again.
    fn template_eof(&mut self) {
        if !self.template_open() {
            self.stopped = true;
            return;
        }
        self.pop_until("template");
        self.clear_to_marker();
        self.tmpl.pop();
        self.reset_mode();
        self.process(Token::Eof);
    }
}
