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
use super::super::ops::scope::Scope;
use super::super::ops::state::Builder;

/// "Clear the stack back to a table body context".
const BODY_CONTEXT: &[&str] = &["tbody", "tfoot", "thead", "template"];
/// Start tags that close the open table section first.
const LEAVE_START: &[&str] = &["caption", "col", "colgroup", "tbody", "tfoot", "thead"];
/// End tags the "in table body" mode ignores.
const IGNORED_END: &[&str] = &["body", "caption", "col", "colgroup", "html", "td", "th", "tr"];

impl Builder {
    /// The "in table body" mode.
    pub(in super::super) fn in_table_body(&mut self, token: Token) {
        match &token {
            Token::Start(t) if t.name == "tr" => {
                self.clear_back_to(BODY_CONTEXT);
                if let Token::Start(t) = token {
                    self.insert_html(t);
                }
                self.mode = Mode::InRow;
            }
            Token::Start(t) if matches!(&*t.name, "th" | "td") => {
                self.clear_back_to(BODY_CONTEXT);
                if let Token::Start(t) = token {
                    self.implied_parent("tr", Mode::InRow, t);
                }
            }
            Token::End(t) if matches!(&*t.name, "tbody" | "tfoot" | "thead") => {
                if self.in_scope(&t.name, Scope::Table) {
                    self.clear_back_to(BODY_CONTEXT);
                    self.pop();
                    self.mode = Mode::InTable;
                }
            }
            Token::Start(t) if LEAVE_START.contains(&&*t.name) => self.leave_table_body(token),
            Token::End(t) if t.name == "table" => self.leave_table_body(token),
            Token::End(t) if IGNORED_END.contains(&&*t.name) => {}
            _ => self.in_table(token),
        }
    }

    /// Close the open table section and let the table handle the token.
    fn leave_table_body(&mut self, token: Token) {
        let open = ["tbody", "thead", "tfoot"].iter().any(|n| self.in_scope(n, Scope::Table));
        if open {
            self.clear_back_to(BODY_CONTEXT);
            self.pop();
            self.reprocess(Mode::InTable, token);
        }
    }
}
