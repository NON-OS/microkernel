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

use alloc::string::String;

use super::state::{is_ws, Tokenizer};
use super::token::{Doctype, Token};

impl<'a> Tokenizer<'a> {
    /// A doctype, the position just past "<!DOCTYPE" (13.2.5.53 to 13.2.5.56).
    /// Anything malformed sets force-quirks; `doctype_id` reads identifiers.
    pub(super) fn doctype(&mut self) -> Token<'a> {
        let mut d = Doctype::default();
        self.skip_ws();
        match self.peek() {
            None => d.force_quirks = true,
            Some(b'>') => {
                self.pos += 1;
                d.force_quirks = true;
            }
            Some(_) => {
                d.name = Some(self.doctype_name());
                self.after_doctype_name(&mut d);
            }
        }
        Token::Doctype(d)
    }

    fn doctype_name(&mut self) -> String {
        let b = self.src.as_bytes();
        let from = self.pos;
        let stop =
            b[from..].iter().position(|&c| is_ws(c) || c == b'>').map_or(b.len(), |n| from + n);
        self.pos = stop;
        let mut name = String::with_capacity(stop - from);
        super::lower::push_lower(&mut name, &self.src[from..stop]);
        name
    }

    fn after_doctype_name(&mut self, d: &mut Doctype) {
        self.skip_ws();
        let rest = &self.src.as_bytes()[self.pos..];
        let keyword = |k: &[u8]| rest.len() >= 6 && rest[..6].eq_ignore_ascii_case(k);
        match rest.first() {
            None => d.force_quirks = true,
            Some(b'>') => self.pos += 1,
            Some(_) if keyword(b"public") => {
                self.pos += 6;
                self.keyword_id(d, true);
            }
            Some(_) if keyword(b"system") => {
                self.pos += 6;
                self.keyword_id(d, false);
            }
            Some(_) => {
                d.force_quirks = true;
                self.skip_past_gt();
            }
        }
    }
}
