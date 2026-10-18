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
//! region, and it lives until the thread that asked for it ends: nothing
//! here unmaps.

use std::cell::RefCell;
use std::sync::atomic::AtomicU64;

/// What the broker writes back for a mapping, field for field.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DmaMapOut {
    pub user_va: u64,
    pub device_addr: u64,
    pub length: u64,
    pub grant_id: u64,
}

const EINVAL: i64 = -22;
/// The largest grant the driver asks for: a 3840x2160 primary surface.
const MAX_GRANT: u64 = 32 << 20;

thread_local! {
    static GRANTS: RefCell<Vec<Box<[AtomicU64]>>> = const { RefCell::new(Vec::new()) };
}

pub fn mk_dma_map(
    _device_id: u64,
    _claim_epoch: u64,
    length: u64,
    _flags: u32,
    out: *mut DmaMapOut,
) -> i64 {
    if length == 0 || length > MAX_GRANT {
        return EINVAL;
    }
    let grant: Box<[AtomicU64]> = (0..length.div_ceil(8)).map(|_| AtomicU64::new(0)).collect();
    let at = grant.as_ptr() as u64;
    let grant_id = GRANTS.with(|g| {
        g.borrow_mut().push(grant);
        g.borrow().len() as u64
    });
    // SAFETY: the driver passes its own DmaMapOut to fill, as it does to the
    // kernel.
    unsafe { *out = DmaMapOut { user_va: at, device_addr: at, length, grant_id } };
    0
}
