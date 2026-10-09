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

use super::super::ops::mode::{Entry, Mode};
use super::super::ops::scope::Scope;
use super::super::ops::state::Builder;

/// "Clear the stack back to a table row context".
const ROW_CONTEXT: &[&str] = &["tr", "template"];

impl Builder {
    /// The "in row" mode.
    pub(in super::super) fn in_row(&mut self, token: Token) {
        let name = match &token {
            Token::Start(t) | Token::End(t) => &*t.name,
            _ => "",
        };
        let start = matches!(token, Token::Start(_));
        match (start, name) {
            (true, "th" | "td") => {
                self.clear_back_to(ROW_CONTEXT);
                if let Token::Start(t) = token {
                    self.insert_html(t);
                }
                self.mode = Mode::InCell;
                self.fmt.push(Entry::Marker);
            }
            (false, "tr") => {
                self.close_row();
            }
            (true, "caption" | "col" | "colgroup" | "tbody" | "tfoot" | "thead" | "tr")
            | (false, "table") => {
                if self.close_row() {
                    self.process(token);
                }
            }
            (false, "tbody" | "tfoot" | "thead") => {
                if self.in_scope(name, Scope::Table) && self.close_row() {
                    self.process(token);
                }
            }
            (false, "body" | "caption" | "col" | "colgroup" | "html" | "td" | "th") => {}
            _ => self.in_table(token),
        }
    }

    /// Close the open row, back to "in table body". False when there is none.
    fn close_row(&mut self) -> bool {
        if !self.in_scope("tr", Scope::Table) {
            return false;
        }
        self.clear_back_to(ROW_CONTEXT);
        self.pop();
        self.mode = Mode::InTableBody;
        true
    }
}
