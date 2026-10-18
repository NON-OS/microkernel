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

//! One item into the walk's state, and the checks a whole descriptor has
//! to pass once its last item is in.

use super::item::{Item, GLOBAL, LOCAL, MAIN};
use super::limits::{Refused, MAX_REPORT_BYTES};
use super::locals::Locals;
use super::main_item::Hand;
use super::walk::Walk;

impl Walk {
    pub(super) fn take(&mut self, item: Item, f: Hand<'_>) -> Result<(), Refused> {
        match item.kind {
            GLOBAL => self.g.take(item),
            LOCAL => {
                self.l.take(item, self.g.now.page);
                Ok(())
            }
            MAIN => {
                let taken = self.main(item, f);
                // Local items last until the next main item.
                self.l = Locals::default();
                taken
            }
            _ => Ok(()),
        }
    }

    /// Every collection closed, no input field the report IDs cannot tell
    /// apart, and every input report, ID byte and all, within
    /// MAX_REPORT_BYTES.
    pub(super) fn finish(&self) -> Result<(), Refused> {
        let ids = self.g.ids;
        let fits = self.inputs.longest(ids) <= MAX_REPORT_BYTES;
        let told_apart = !(ids && self.inputs.unnumbered());
        (self.depth == 0 && told_apart && fits).then_some(()).ok_or(Refused)
    }
}
