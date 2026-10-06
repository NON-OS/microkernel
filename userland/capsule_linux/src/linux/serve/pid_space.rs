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

//! Where a family's pid numbers are kept. A personality hosts one family, so
//! they live in one static any handler can reach without the family at hand.

use alloc::vec::Vec;
use core::cell::RefCell;

pub(super) struct Space(pub(super) RefCell<(Vec<(u32, u32)>, u32)>);
/* SAFETY: eK@nonos.systems - only the personality's single serve loop touches it. */
unsafe impl Sync for Space {}
pub(super) static SPACE: Space = Space(RefCell::new((Vec::new(), 0)));

/// `PidNs::inward`, for a handler with no family at hand.
pub fn inward(g: u32) -> Option<u32> {
    let space = SPACE.0.borrow();
    space.0.iter().find(|(_, gp)| *gp == g).map(|(k, _)| *k)
}

/// `PidNs::outward`, for a handler with no family at hand.
pub fn outward(k: u32) -> u32 {
    let mut space = SPACE.0.borrow_mut();
    if let Some(&(_, g)) = space.0.iter().find(|(kp, _)| *kp == k) {
        return g;
    }
    let g = space.1;
    let Some(after) = g.checked_add(1) else {
        return 0;
    };
    space.1 = after;
    space.0.push((k, g));
    g
}
