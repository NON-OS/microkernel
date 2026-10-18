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

//! An Input or Feature item's fields, handed on one by one. Constant fields
//! are padding and array fields select usages: both have taken their bits,
//! and neither is handed on; nor is a field wider than this driver reads.

use super::field_at::{FieldAt, MAX_READ_BITS};
use super::main_item::Hand;
use super::walk::Walk;

const CONSTANT: u32 = 1 << 0;
const VARIABLE: u32 = 1 << 1;
const RELATIVE: u32 = 1 << 2;

impl Walk {
    /// Each variable field of the item laid out from bit `at` of its report.
    pub(super) fn hand(&self, at: u32, flags: u32, feature: bool, f: Hand<'_>) {
        let g = self.g.now;
        let data = flags & CONSTANT == 0 && flags & VARIABLE != 0 && self.depth > 0;
        if !data || !(1..=MAX_READ_BITS).contains(&g.size) {
            return;
        }
        let relative = flags & RELATIVE != 0;
        // Past its distinct usages a field repeats the last one's usage, which
        // every reader takes the first of.
        for i in 0..g.count.min(self.l.distinct()) {
            let Some(start) = i.checked_mul(g.size).and_then(|o| at.checked_add(o)) else { return };
            let usage = self.l.usage(i);
            let (app, id, min, max, size) = (self.app, g.id, g.min, g.max, g.size);
            f(&FieldAt { feature, app, id, usage, at: start, size, min, max, relative });
        }
    }
}
