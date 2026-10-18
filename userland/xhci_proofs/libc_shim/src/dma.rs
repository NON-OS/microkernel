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

//! DMA grants in host memory, page aligned and zeroed as the kernel's are,
//! so the driver's `DmaPool` and every ring on it run unchanged. Giving one
//! back is `unmap`.

use std::alloc::{alloc_zeroed, Layout};
use std::sync::atomic::{AtomicU64, Ordering};

use crate::dma_refuse::refused;
use crate::grants::{bus_of, grants, Grant, BUS_STRIDE, PAGE};

pub(crate) const EINVAL: i64 = -22;
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

static NEXT_GRANT: AtomicU64 = AtomicU64::new(1);

/// Map `length` bytes, a whole number of pages, as the kernel would.
pub fn mk_dma_map(_device: u64, _epoch: u64, length: u64, _flags: u32, out: &mut DmaMapOut) -> i64 {
    if length == 0 || !length.is_multiple_of(PAGE) || length > BUS_STRIDE {
        return EINVAL;
    }
    if refused() {
        return ENOMEM;
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
    let bus = bus_of(id);
    grants().push(Grant { id, host: host as u64, bus, len: length });
    *out = DmaMapOut { user_va: host as u64, device_addr: bus, length, grant_id: id };
    0
}
