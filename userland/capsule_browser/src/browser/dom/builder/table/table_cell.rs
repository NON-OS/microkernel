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

impl Builder {
    /// The "in cell" mode.
    pub(in super::super) fn in_cell(&mut self, token: Token) {
        let name = match &token {
            Token::Start(t) | Token::End(t) => &*t.name,
            _ => "",
        };
        let start = matches!(token, Token::Start(_));
        match (start, name) {
            (false, "td" | "th") => {
                if self.in_scope(name, Scope::Table) {
                    self.implied_end(None);
                    self.pop_until(name);
                    self.clear_to_marker();
                    self.mode = Mode::InRow;
                }
            }
            (
                true,
                "caption" | "col" | "colgroup" | "tbody" | "td" | "tfoot" | "th" | "thead" | "tr",
            ) => {
                if self.in_scope("td", Scope::Table) || self.in_scope("th", Scope::Table) {
                    self.close_cell();
                    self.process(token);
                }
            }
            (false, "body" | "caption" | "col" | "colgroup" | "html") => {}
            (false, "table" | "tbody" | "tfoot" | "thead" | "tr") => {
                if self.in_scope(name, Scope::Table) {
                    self.close_cell();
                    self.process(token);
                }
            }
            _ => self.in_body(token),
        }
    }

    fn close_cell(&mut self) {
        self.implied_end(None);
        self.pop_until_any(&["td", "th"]);
        self.clear_to_marker();
        self.mode = Mode::InRow;
    }
}
