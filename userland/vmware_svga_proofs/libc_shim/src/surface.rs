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

//! Anonymous memory and the surface registry.

use std::alloc::{alloc_zeroed, Layout};
use std::cell::{Cell, RefCell};

pub const SURFACE_FORMAT_ARGB8888: u32 = 1;

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
    pub owner_pid: u32,
    pub writable: u32,
}

thread_local! {
    static SURFACES: RefCell<Vec<SurfaceDescriptor>> = const { RefCell::new(Vec::new()) };
    static UNMAPPED: RefCell<Vec<u64>> = const { RefCell::new(Vec::new()) };
    static REFUSE: Cell<bool> = const { Cell::new(false) };
}

pub fn fail_surface() {
    REFUSE.with(|r| r.set(true));
}

/// Every surface registered, and every address unmapped, on this thread.
pub fn surfaces() -> (Vec<SurfaceDescriptor>, Vec<u64>) {
    (SURFACES.with(|s| s.borrow().clone()), UNMAPPED.with(|u| u.borrow().clone()))
}

pub fn mk_mmap(_a: *mut u8, len: usize, _p: i32, _f: i32, _fd: i32, _o: i64) -> *mut u8 {
    // SAFETY: a non-zero, page-aligned layout; the memory is leaked on
    // purpose, as a mapping outlives the call that made it.
    unsafe { alloc_zeroed(Layout::from_size_align(len, 4096).unwrap()) }
}

pub fn mk_munmap(addr: *mut u8, _len: usize) -> i64 {
    UNMAPPED.with(|u| u.borrow_mut().push(addr as u64));
    0
}

pub fn mk_surface_register(desc: *const SurfaceDescriptor) -> i64 {
    if REFUSE.with(|r| r.get()) {
        return -12;
    }
    // SAFETY: the caller hands a live descriptor.
    SURFACES.with(|s| s.borrow_mut().push(unsafe { *desc }));
    3
}
