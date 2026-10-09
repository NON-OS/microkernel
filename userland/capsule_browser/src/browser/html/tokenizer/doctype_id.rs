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

use super::state::Tokenizer;
use super::token::Doctype;

impl<'a> Tokenizer<'a> {
    /// After PUBLIC or SYSTEM: the quoted identifier (13.2.5.57 to 13.2.5.67).
    pub(super) fn keyword_id(&mut self, d: &mut Doctype, public: bool) {
        self.skip_ws();
        match self.peek() {
            Some(q @ (b'"' | b'\'')) => {
                self.pos += 1;
                let (id, closed) = self.quoted_id(q);
                if public {
                    d.public_id = Some(id);
                } else {
                    d.system_id = Some(id);
                }
                match (closed, public) {
                    (false, _) => d.force_quirks = true,
                    (true, true) => self.after_public_id(d),
                    (true, false) => self.after_system_id(d),
                }
            }
            Some(b'>') => {
                self.pos += 1;
                d.force_quirks = true;
            }
            None => d.force_quirks = true,
            Some(_) => {
                d.force_quirks = true;
                self.skip_past_gt();
            }
        }
    }

    /// A public identifier may be followed by a system one.
    fn after_public_id(&mut self, d: &mut Doctype) {
        self.skip_ws();
        match self.peek() {
            Some(b'"' | b'\'') => self.keyword_id(d, false),
            Some(b'>') => self.pos += 1,
            None => d.force_quirks = true,
            Some(_) => {
                d.force_quirks = true;
                self.skip_past_gt();
            }
        }
    }
}
