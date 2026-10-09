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

//! DMA grants in host memory, so the driver's own `DmaPool` and every ring
//! built on it run unchanged.
//!
//! A grant is page aligned and zeroed, as the kernel's are. Its bus address
//! is not its host address: each grant has its own window above 4 GiB, so a
//! driver that hands the controller a virtual address, or keeps only the low
//! half of a bus address, fails a test instead of happening to work. A test
//! plays the controller through [`dma_host`], which turns a bus address back
//! into the host memory behind it, the way the controller's DMA would reach
//! it.

use std::alloc::{alloc_zeroed, dealloc, Layout};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;

const PAGE: u64 = 4096;
const BUS_BASE: u64 = 0x0000_00F0_0000_0000;
/// Room between grants on the bus. The driver asks for 16 pages at most.
const BUS_STRIDE: u64 = 1 << 20;
const EINVAL: i64 = -22;
const ENOMEM: i64 = -12;

/// What the broker writes back for a mapping, field for field.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DmaMapOut {
    pub user_va: u64,
    pub device_addr: u64,
    pub length: u64,
    pub grant_id: u64,
}

struct Grant {
    id: u64,
    host: u64,
    bus: u64,
    len: u64,
}

static GRANTS: Mutex<Vec<Grant>> = Mutex::new(Vec::new());
static NEXT_GRANT: AtomicU64 = AtomicU64::new(1);

fn grants() -> std::sync::MutexGuard<'static, Vec<Grant>> {
    GRANTS.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

/// Map `length` bytes, a whole number of pages, as the kernel would.
pub fn mk_dma_map(
    _device_id: u64,
    _claim_epoch: u64,
    length: u64,
    _flags: u32,
    out: &mut DmaMapOut,
) -> i64 {
    if length == 0 || length % PAGE != 0 || length > BUS_STRIDE {
        return EINVAL;
    }
    let Ok(layout) = Layout::from_size_align(length as usize, PAGE as usize) else {
        return EINVAL;
    };
    // SAFETY: the layout has a non-zero size.
    let host = unsafe { alloc_zeroed(layout) };
    if host.is_null() {
        return ENOMEM;
    }
    let id = NEXT_GRANT.fetch_add(1, Ordering::Relaxed);
    let bus = BUS_BASE + id * BUS_STRIDE;
    grants().push(Grant { id, host: host as u64, bus, len: length });
    *out = DmaMapOut { user_va: host as u64, device_addr: bus, length, grant_id: id };
    0
}

pub fn mk_dma_unmap(grant_id: u64) -> i64 {
    let grant = {
        let mut all = grants();
        let Some(at) = all.iter().position(|g| g.id == grant_id) else {
            return EINVAL;
        };
        all.swap_remove(at)
    };
    // SAFETY: the grant was allocated by `mk_dma_map` with this layout and is
    // freed once, having just left the table.
    unsafe {
        dealloc(
            grant.host as *mut u8,
            Layout::from_size_align_unchecked(grant.len as usize, PAGE as usize),
        );
    }
    0
}

/// The host address behind bus address `bus`, or `None` when no live grant
/// covers it: what the controller reaches when it follows a pointer.
pub fn dma_host(bus: u64) -> Option<u64> {
    grants()
        .iter()
        .find(|g| bus >= g.bus && bus - g.bus < g.len)
        .map(|g| g.host + (bus - g.bus))
}
