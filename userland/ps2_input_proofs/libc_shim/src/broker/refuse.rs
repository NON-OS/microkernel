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

//! The interrupt lines the broker will not bind on this thread, for a
//! machine whose IO-APIC has no route for one. `present` clears them.

use std::cell::RefCell;

thread_local! {
    static REFUSED: RefCell<Vec<u32>> = const { RefCell::new(Vec::new()) };
}

/// From now on `mk_irq_bind` refuses each of `lines`.
pub fn refuse_lines(lines: &[u32]) {
    REFUSED.with(|r| *r.borrow_mut() = lines.to_vec());
}

pub(super) fn refused(line: u32) -> bool {
    REFUSED.with(|r| r.borrow().contains(&line))
}
