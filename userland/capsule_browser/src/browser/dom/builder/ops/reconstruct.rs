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

use super::super::super::limits::MAX_DEPTH;
use super::mode::Entry;
use super::state::Builder;

impl Builder {
    /// Reconstruct the active formatting elements (13.2.4.3): reopen, in
    /// order, every entry after the last marker that is no longer open, so
    /// `<p><b>x</p>y` keeps "y" bold. Stops at the stack cap rather than
    /// closing what it just opened.
    pub(in super::super) fn reconstruct(&mut self) {
        let open = |b: &Builder, e: Entry| match e {
            Entry::Marker => true,
            Entry::Elem(id) => b.on_stack(id),
        };
        match self.fmt.last() {
            Some(&e) if !open(self, e) => {}
            _ => return,
        }
        let mut i = self.fmt.len() - 1;
        while i > 0 && !open(self, self.fmt[i - 1]) {
            i -= 1;
        }
        while i < self.fmt.len() && self.open.len() < MAX_DEPTH {
            let Entry::Elem(old) = self.fmt[i] else {
                return;
            };
            let Some(new) = self.insert_html(self.token_of(old)) else {
                return;
            };
            self.replace_fmt(i, old, new);
            i += 1;
        }
    }
}
