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

//! The device's side of the DMA grants: `len` bytes at a device address,
//! inside one live grant, while bus mastering is on. With it off the
//! access never leaves the device and is counted as blocked. Inside no
//! live grant it is a violation, counted and noted: the frame-after-free
//! C2 forbids, or a transfer past a buffer.

use crate::claim::bus_master;
use crate::grants::GRANTS;
use crate::tally::{count_blocked, count_violation};

fn reach(iova: u64, len: usize) -> Option<*mut u8> {
    if !bus_master() {
        count_blocked(iova, len);
        return None;
    }
    let at = GRANTS.with(|g| {
        g.borrow().iter().find_map(|x| {
            let off = iova.checked_sub(x.iova)? as usize;
            (x.live && off.checked_add(len)? <= x.len).then(|| x.ptr.wrapping_add(off))
        })
    });
    if at.is_none() {
        count_violation(iova, len);
    }
    at
}

/// What the device reads from memory, `None` when it cannot reach it.
pub fn dma_read(iova: u64, len: usize) -> Option<Vec<u8>> {
    let p = reach(iova, len)?;
    // SAFETY: `reach` found `len` bytes from `p` inside a live grant's
    // leaked allocation, which is never freed.
    Some(unsafe { std::slice::from_raw_parts(p, len) }.to_vec())
}

/// What the device writes to memory; false, and nothing written, when it
/// cannot reach it.
pub fn dma_write(iova: u64, data: &[u8]) -> bool {
    let Some(p) = reach(iova, data.len()) else {
        return false;
    };
    // SAFETY: as dma_read.
    unsafe { std::ptr::copy_nonoverlapping(data.as_ptr(), p, data.len()) };
    true
}
