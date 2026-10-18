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

//! DMA grants in host memory, zeroed as the kernel's are. A grant's device
//! address is its host address, as the harness makes them for the ring
//! region, and every grant is kept per thread until unmapped or the test
//! resets, so a test sees which are still held.

use std::cell::RefCell;
use std::sync::atomic::AtomicU64;

pub(crate) const EINVAL: i64 = -22;
/// The largest grant the driver asks for: one 3840x2160 surface.
const MAX_GRANT: u64 = 32 << 20;
pub const MK_DMA_MAP_HIGH: u32 = 1 << 0;

/// What the broker writes back for a mapping, field for field.
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DmaMapOut {
    pub user_va: u64,
    pub device_addr: u64,
    pub length: u64,
    pub grant_id: u64,
}

pub(crate) type Grant = (u64, Box<[AtomicU64]>);

thread_local! {
    pub(crate) static GRANTS: RefCell<Vec<Grant>> = const { RefCell::new(Vec::new()) };
}

pub fn mk_dma_map(_device: u64, _epoch: u64, length: u64, _flags: u32, out: *mut DmaMapOut) -> i64 {
    if length == 0 || length > MAX_GRANT {
        return EINVAL;
    }
    let words: Box<[AtomicU64]> = (0..length.div_ceil(8)).map(|_| AtomicU64::new(0)).collect();
    let at = words.as_ptr() as u64;
    let grant_id = GRANTS.with(|g| {
        let mut g = g.borrow_mut();
        let id = g.iter().map(|e| e.0).max().unwrap_or(0) + 1;
        g.push((id, words));
        id
    });
    // SAFETY: the driver passes its own DmaMapOut to fill, as to the kernel.
    unsafe { *out = DmaMapOut { user_va: at, device_addr: at, length, grant_id } };
    0
}
