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

use super::super::ops::mode::Mode;
use super::super::ops::scope::Scope;
use super::super::ops::state::Builder;

/// Where characters in a table are collected rather than inserted.
const TABLE_TEXT_PARENTS: &[&str] = &["table", "tbody", "template", "tfoot", "thead", "tr"];

impl Builder {
    /// The "in table" mode (13.2.6.4.9).
    pub(in super::super) fn in_table(&mut self, token: Token) {
        match token {
            Token::Chars(_) | Token::Null if self.cur_is_any(TABLE_TEXT_PARENTS) => {
                self.table_text.clear();
                self.orig = self.mode;
                self.reprocess(Mode::InTableText, token);
            }
            Token::Comment | Token::Doctype(_) => {}
            Token::Start(t) => self.table_start(t),
            Token::End(t) => match &*t.name {
                "table" => {
                    if self.in_scope("table", Scope::Table) {
                        self.pop_until("table");
                        self.reset_mode();
                    }
                }
                "body" | "caption" | "col" | "colgroup" | "html" | "tbody" | "td" | "tfoot"
                | "th" | "thead" | "tr" => {}
                "template" => self.in_head(Token::End(t)),
                _ => self.foster_body(Token::End(t)),
            },
            Token::Eof => self.in_body(Token::Eof),
            other => self.foster_body(other),
        }
    }

    /// "Anything else" in a table: body rules with foster parenting, so
    /// stray content lands before the table instead of inside it.
    pub(in super::super) fn foster_body(&mut self, token: Token) {
        self.foster = true;
        self.in_body(token);
        self.foster = false;
    }

    /// A tag the table rules answer with an implied parent: insert `parent`,
    /// switch to `mode` and handle the tag there.
    pub(in super::super) fn implied_parent(&mut self, parent: &'static str, mode: Mode, t: Tag) {
        self.insert_html(Tag::implied(parent));
        self.reprocess(mode, Token::Start(t));
    }
}
