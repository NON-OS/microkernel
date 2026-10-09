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

/// Start tags that close the caption and go on to the table.
const CLOSERS: &[&str] =
    &["caption", "col", "colgroup", "tbody", "td", "tfoot", "th", "thead", "tr"];
/// End tags the "in caption" mode ignores.
const IGNORED: &[&str] =
    &["body", "col", "colgroup", "html", "tbody", "td", "tfoot", "th", "thead", "tr"];

impl Builder {
    /// The "in caption" mode.
    pub(in super::super) fn in_caption(&mut self, token: Token) {
        let (close, again) = match &token {
            Token::End(t) if t.name == "caption" => (true, false),
            Token::End(t) if t.name == "table" => (true, true),
            Token::Start(t) => (CLOSERS.contains(&&*t.name), true),
            Token::End(t) if IGNORED.contains(&&*t.name) => return,
            _ => (false, false),
        };
        if !close {
            self.in_body(token);
            return;
        }
        if !self.in_scope("caption", Scope::Table) {
            return;
        }
        self.implied_end(None);
        self.pop_until("caption");
        self.clear_to_marker();
        self.mode = Mode::InTable;
        if again {
            self.process(token);
        }
    }
}
