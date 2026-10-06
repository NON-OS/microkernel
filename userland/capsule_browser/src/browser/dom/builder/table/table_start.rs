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

use super::super::ops::mode::{Entry, Mode};
use super::super::ops::scope::Scope;
use super::super::ops::state::Builder;

/// "Clear the stack back to a table context".
const TABLE_CONTEXT: &[&str] = &["table", "template"];

impl Builder {
    /// A start tag in the "in table" mode.
    pub(in super::super) fn table_start(&mut self, t: Tag) {
        match &*t.name {
            "caption" => {
                self.clear_back_to(TABLE_CONTEXT);
                self.insert_html(t);
                self.fmt.push(Entry::Marker);
                self.mode = Mode::InCaption;
            }
            "colgroup" | "tbody" | "tfoot" | "thead" => {
                self.clear_back_to(TABLE_CONTEXT);
                let body = t.name != "colgroup";
                self.insert_html(t);
                self.mode = if body { Mode::InTableBody } else { Mode::InColumnGroup };
            }
            "col" => {
                self.clear_back_to(TABLE_CONTEXT);
                self.implied_parent("colgroup", Mode::InColumnGroup, t);
            }
            "td" | "th" | "tr" => {
                self.clear_back_to(TABLE_CONTEXT);
                self.implied_parent("tbody", Mode::InTableBody, t);
            }
            "table" => {
                if self.in_scope("table", Scope::Table) {
                    self.pop_until("table");
                    self.reset_mode();
                    self.process(Token::Start(t));
                }
            }
            "style" | "script" | "template" => self.in_head(Token::Start(t)),
            "input" if t.attr("type").is_some_and(|v| v.eq_ignore_ascii_case("hidden")) => {
                self.insert_void(t);
            }
            "form" => {
                if self.form.is_some() && !self.parsing_template() {
                    return;
                }
                let id = self.insert_void(t);
                if !self.parsing_template() {
                    self.form = id;
                }
            }
            _ => self.foster_body(Token::Start(t)),
        }
    }
}
