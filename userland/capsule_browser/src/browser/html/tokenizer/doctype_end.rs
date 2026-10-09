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

use super::state::Tokenizer;
use super::token::Doctype;

impl<'a> Tokenizer<'a> {
    /// The identifier up to the closing quote, and whether that quote came.
    /// A ">" first ends the whole doctype there.
    pub(super) fn quoted_id(&mut self, quote: u8) -> (String, bool) {
        let b = self.src.as_bytes();
        let from = self.pos;
        let stop = b[from..].iter().position(|&c| c == quote || c == b'>');
        let end = stop.map_or(b.len(), |n| from + n);
        let mut id = String::with_capacity(end - from);
        super::lower::push_text(&mut id, &self.src[from..end]);
        self.pos = (end + 1).min(b.len());
        (id, b.get(end) == Some(&quote))
    }

    /// Anything after the system identifier is ignored, without quirks.
    pub(super) fn after_system_id(&mut self, d: &mut Doctype) {
        self.skip_ws();
        match self.peek() {
            Some(b'>') => self.pos += 1,
            None => d.force_quirks = true,
            Some(_) => self.skip_past_gt(),
        }
    }

    /// Skip past the next ">", or to the end: how a bogus comment ends, and
    /// the rest of a doctype that makes no sense.
    pub(super) fn skip_past_gt(&mut self) {
        let b = self.src.as_bytes();
        self.pos = match b[self.pos..].iter().position(|&c| c == b'>') {
            Some(n) => self.pos + n + 1,
            None => b.len(),
        };
    }
}
