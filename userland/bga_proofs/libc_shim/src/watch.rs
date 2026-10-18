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

//! Which BAR holds the adapter's registers, and the run of them each
//! give-back records (`registers_at_give_back`).

use std::cell::RefCell;
use std::ops::Range;

use crate::mmio::{mapping, GRANT_BASE};
use crate::seal;

thread_local! {
    static WATCHED: RefCell<(u32, Range<usize>)> = const { RefCell::new((u32::MAX, 0..0)) };
}

/// BAR `bar` holds the adapter's registers: its mapping is sealed when it
/// is given back (`seal`), and `block` is the run of them each give-back
/// records.
pub fn watch(bar: u8, block: Range<usize>) {
    WATCHED.with(|w| *w.borrow_mut() = (bar as u32, block));
    let _ = seal::forget();
}

/// Whether `grant` is the register mapping's.
pub(crate) fn watches(grant: u64) -> bool {
    grant.checked_sub(GRANT_BASE) == Some(WATCHED.with(|w| w.borrow().0) as u64)
}

/// The register mapping, its length, and the watched block within it.
pub(crate) fn registers() -> Option<(u64, usize, Range<usize>)> {
    let (bar, block) = WATCHED.with(|w| w.borrow().clone());
    let (va, len) = mapping(bar)?;
    let len = len as usize;
    Some((va, len, block.start.min(len)..block.end.min(len)))
}
