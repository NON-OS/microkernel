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

//! MkSurfaceRegister: each descriptor the included bring-up registers is
//! kept for the test to read, and answered with the next sid.

use std::cell::RefCell;

pub const SURFACE_FORMAT_ARGB8888: u32 = 1;

/// The kernel's 48-byte descriptor, field for field (K20).
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

const _: () = assert!(core::mem::size_of::<SurfaceDescriptor>() == 48);

/// The sid the first registration on a thread is answered with.
pub const FIRST_SID: i64 = 0x40;

thread_local! {
    static REGISTERED: RefCell<Vec<SurfaceDescriptor>> = const { RefCell::new(Vec::new()) };
}

pub fn mk_surface_register(desc: *const SurfaceDescriptor) -> i64 {
    // SAFETY: the caller hands its own live descriptor, as to the kernel.
    let d = unsafe { *desc };
    REGISTERED.with(|r| {
        r.borrow_mut().push(d);
        FIRST_SID + r.borrow().len() as i64 - 1
    })
}

/// Every descriptor registered on this thread, in order.
pub fn registered() -> Vec<SurfaceDescriptor> {
    REGISTERED.with(|r| r.borrow().clone())
}
