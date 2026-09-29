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

use super::limits::MAX_TAG_ATTRS;
use super::lower::push_lower;
use super::state::{is_ws, Tokenizer};
use super::token::Tag;

impl<'a> Tokenizer<'a> {
    /// One attribute, the position at the first character of its name. None
    /// when the input ends inside it.
    ///
    /// A name runs to whitespace, "/", ">" or "=", so `x<y`, `"a` and `@click`
    /// are names; a first "=" belongs to the name. The first of two
    /// attributes with one name wins. Only a start tag's are kept, and at
    /// most `MAX_TAG_ATTRS` of them.
    pub(super) fn attribute(&mut self, tag: &mut Tag, keep: bool) -> Option<()> {
        let name = self.attr_name();
        self.skip_ws();
        let mut value = String::new();
        let mut whole = true;
        if self.peek() == Some(b'=') {
            self.pos += 1;
            self.skip_ws();
            whole = self.attr_value(&mut value)?;
        }
        if !keep || tag.attrs.iter().any(|(n, _)| *n == name) {
            return Some(());
        }
        if !whole || tag.attrs.len() >= MAX_TAG_ATTRS {
            self.dropped_attrs = true;
            return Some(());
        }
        value.shrink_to_fit();
        tag.attrs.push((name, value));
        Some(())
    }

    fn attr_name(&mut self) -> String {
        let b = self.src.as_bytes();
        let from = self.pos;
        let stop = b[from + 1..]
            .iter()
            .position(|&c| is_ws(c) || matches!(c, b'/' | b'>' | b'='))
            .map_or(b.len(), |n| from + 1 + n);
        self.pos = stop;
        let mut name = String::with_capacity(stop - from);
        push_lower(&mut name, &self.src[from..stop]);
        name
    }
}
