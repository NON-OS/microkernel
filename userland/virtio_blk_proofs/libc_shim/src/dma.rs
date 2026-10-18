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

//! DMA grants: host memory, page aligned and zeroed, at a device address
//! below 4 GiB so a legacy ring's page number fits its register.

use std::cell::{Cell, RefCell};

use crate::events::{record, Event};
use crate::types::dma::DmaMapOut;

const PAGE: u64 = 4096;

thread_local! {
    static NEXT_PHYS: Cell<u64> = const { Cell::new(0x10_0000) };
    /// Every grant made: its device address, where it is, and its length.
    static GRANTS: RefCell<Vec<(u64, u64, u64)>> = const { RefCell::new(Vec::new()) };
}

/// Where the `len` bytes a device reaches at `phys` are, if one grant
/// holds them all.
pub(crate) fn locate(phys: u64, len: usize) -> Option<*mut u8> {
    GRANTS.with(|g| {
        let g = g.borrow();
        let &(base, va, size) = g.iter().find(|(b, _, s)| (*b..b + s).contains(&phys))?;
        let off = phys - base;
        (off + len as u64 <= size).then_some((va + off) as *mut u8)
    })
}

pub fn mk_dma_map(_d: u64, _e: u64, length: u64, _flags: u32, out: *mut DmaMapOut) -> i64 {
    record(Event::DmaMap(length));
    let len = length.div_ceil(PAGE).max(1) * PAGE;
    let Ok(layout) = std::alloc::Layout::from_size_align(len as usize, PAGE as usize) else {
        return -22;
    };
    // SAFETY: the layout is non-zero in size. The memory is never freed: a
    // grant lives until the test's process ends, as long as any view of it.
    let va = unsafe { std::alloc::alloc_zeroed(layout) } as u64;
    if va == 0 {
        return -12;
    }
    let phys = NEXT_PHYS.with(|n| n.replace(n.get() + len));
    GRANTS.with(|g| g.borrow_mut().push((phys, va, len)));
    // SAFETY: the driver passes the address of one of its own records.
    unsafe { *out = DmaMapOut { user_va: va, device_addr: phys, length: len, grant_id: 9 } };
    0
}
