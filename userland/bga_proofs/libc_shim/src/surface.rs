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

//! The kernel's surface registry, per thread: each surface registered, as
//! its descriptor, and a switch that refuses the next ones.

use std::cell::{Cell, RefCell};

pub const SURFACE_FORMAT_ARGB8888: u32 = 1;
/// What the share answers for the surface registered as `sid`.
pub const SHARED_HANDLE_BASE: i64 = 0x7700;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SurfaceDescriptor {
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub format: u32,
    pub byte_len: u64,
    pub base_va: u64,
    pub flags: u64,
    /// Written by an attach: the process that registered the surface.
    pub owner_pid: u32,
    /// Written by an attach: 1 when the view is writable.
    pub writable: u32,
}

thread_local! {
    static REGISTERED: RefCell<Vec<SurfaceDescriptor>> = const { RefCell::new(Vec::new()) };
    static REFUSE: Cell<bool> = const { Cell::new(false) };
}

/// Every surface registered on this thread, in order.
pub fn registered() -> Vec<SurfaceDescriptor> {
    REGISTERED.with(|r| r.borrow().clone())
}

/// Whether the registry refuses surfaces from now on.
pub fn refuse_surfaces(refuse: bool) {
    REFUSE.with(|r| r.set(refuse));
}

pub fn mk_surface_register(desc: *const SurfaceDescriptor) -> i64 {
    if REFUSE.with(Cell::get) {
        return -22;
    }
    // SAFETY: the caller hands a descriptor it holds, as the real call
    // requires.
    let desc = unsafe { *desc };
    REGISTERED.with(|r| {
        r.borrow_mut().push(desc);
        r.borrow().len() as i64 - 1
    })
}
