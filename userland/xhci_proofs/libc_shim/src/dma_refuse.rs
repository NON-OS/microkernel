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

//! A broker with no DMA memory left to give: a test has one of this
//! thread's mappings refused, so the driver's way out of an allocation
//! that fails part-way can be seen.

use std::cell::Cell;

thread_local! {
    /// Mappings still granted on this thread before one is refused.
    static LEFT: Cell<Option<u32>> = const { Cell::new(None) };
}

/// Grant `n` more mappings on this thread, then refuse the next with
/// ENOMEM; those after it are granted again.
pub fn refuse_dma_after(n: u32) {
    LEFT.with(|c| c.set(Some(n)));
}

/// Whether the mapping asked for now is the one to refuse.
pub(crate) fn refused() -> bool {
    LEFT.with(|c| match c.get() {
        Some(0) => {
            c.set(None);
            true
        }
        Some(n) => {
            c.set(Some(n - 1));
            false
        }
        None => false,
    })
}
