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

use alloc::borrow::Cow;
use core::mem;

use crate::browser::html::tokenizer::Token;

use super::super::ops::chars::all_space;
use super::super::ops::state::Builder;

impl Builder {
    /// The "in table text" mode: whitespace stays in the table, anything
    /// else is fostered out as a whole.
    pub(in super::super) fn in_table_text(&mut self, token: Token) {
        match token {
            Token::Null => {}
            Token::Chars(s) => self.table_text.push_str(&s),
            other => {
                let pending = mem::take(&mut self.table_text);
                if all_space(&pending) {
                    self.insert_text(Cow::Owned(pending));
                } else {
                    self.foster_body(Token::Chars(Cow::Owned(pending)));
                }
                self.reprocess(self.orig, other);
            }
        }
    }
}
