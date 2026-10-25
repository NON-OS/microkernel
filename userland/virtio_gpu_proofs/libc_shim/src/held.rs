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

//! The grants a test reads back, and their giving back: one by the driver's
//! own unmap, or all of a thread's when a test starts afresh.

use crate::dma::{EINVAL, GRANTS};

pub fn mk_dma_unmap(grant_id: u64) -> i64 {
    GRANTS.with(|g| {
        let mut g = g.borrow_mut();
        let before = g.len();
        g.retain(|e| e.0 != grant_id);
        if g.len() < before {
            0
        } else {
            EINVAL
        }
    })
}

/// Each grant still held, as (id, first address, bytes).
pub fn dma_spans() -> Vec<(u64, u64, u64)> {
    GRANTS.with(|g| {
        g.borrow().iter().map(|e| (e.0, e.1.as_ptr() as u64, e.1.len() as u64 * 8)).collect()
    })
}

/// Every grant of this thread given back, its memory freed.
pub fn dma_reset() {
    GRANTS.with(|g| g.borrow_mut().clear());
}
