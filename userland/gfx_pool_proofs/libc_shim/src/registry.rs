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

//! The surface registry, by the rules the kernel's handlers state: register
//! names frames of the caller's own mapping and answers a sid, never the
//! handle; a share answers the handle, and to the caller's own process
//! records no right; release takes the owner's handle. Each entry keeps
//! whether its bytes were zero when they were registered.

use std::cell::{Cell, RefCell};

use crate::mem_read::bytes_at;
use crate::surface_registry::SurfaceDescriptor;

#[derive(Clone, Debug)]
pub struct Entry {
    pub sid: u64,
    pub handle: u64,
    pub base: usize,
    pub len: usize,
    /// Every byte zero at registration: what a receiver sees first.
    pub zero: bool,
    /// Each process it was shared with, and the share's flags.
    pub shares: Vec<(u32, u64)>,
}

thread_local! {
    pub(crate) static LIVE: RefCell<Vec<Entry>> = const { RefCell::new(Vec::new()) };
    static NEXT: Cell<u64> = const { Cell::new(1) };
    pub(crate) static NO_REGISTER: Cell<bool> = const { Cell::new(false) };
    pub(crate) static NO_SHARE: Cell<bool> = const { Cell::new(false) };
    pub(crate) static DIRTY: Cell<u32> = const { Cell::new(0) };
}

pub fn mk_surface_register(desc: &SurfaceDescriptor) -> i64 {
    let (base, len) = (desc.base_va as usize, desc.byte_len as usize);
    let Some(bytes) = bytes_at(base, len).filter(|_| !NO_REGISTER.with(Cell::get)) else {
        return -14;
    };
    let sid = NEXT.with(|n| n.replace(n.get() + 1));
    let zero = bytes.iter().all(|&b| b == 0);
    DIRTY.with(|d| d.set(d.get() + !zero as u32));
    let e = Entry { sid, handle: sid | 0x1_0000, base, len, zero, shares: Vec::new() };
    LIVE.with(|l| l.borrow_mut().push(e));
    sid as i64
}

pub fn mk_surface_release(handle: u64) -> i64 {
    LIVE.with(|l| {
        let mut l = l.borrow_mut();
        let before = l.len();
        l.retain(|e| e.handle != handle);
        if l.len() < before {
            0
        } else {
            -1
        }
    })
}
