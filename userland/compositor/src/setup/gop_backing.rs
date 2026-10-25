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

//! The buffer a GOP-mode compositor draws into: the boot display's size,
//! an anonymous mapping of its own, registered as a surface and attached
//! back so the kernel's present path resolves it.

use nonos_libc::{mk_mmap, mk_surface_attach, mk_surface_register, nonos_display_dimensions};
use nonos_libc::{SurfaceDescriptor, SURFACE_FORMAT_ARGB8888};

use crate::state::Primary;

const PROT_READ_WRITE: i32 = 0x3;
const MAP_PRIVATE_ANON: i32 = 0x22;

pub fn display_dimensions() -> Result<(u32, u32), &'static str> {
    let mut width: u32 = 0;
    let mut height: u32 = 0;
    let rc = nonos_display_dimensions(0, &mut width as *mut u32, &mut height as *mut u32);
    if rc < 0 || width == 0 || height == 0 {
        return Err("gop: no display dimensions");
    }
    Ok((width, height))
}

// Its own anonymous mapping, not the heap: mmap gives a page-aligned VMA at
// base_va, which the present path resolves; a heap allocation never would.
fn alloc_backing(byte_len: u64) -> Result<u64, &'static str> {
    let size = usize::try_from(byte_len).map_err(|_| "gop: backing too large")?;
    let ptr = mk_mmap(core::ptr::null_mut(), size, PROT_READ_WRITE, MAP_PRIVATE_ANON, -1, 0);
    if ptr.is_null() {
        return Err("gop: backing mmap failed");
    }
    Ok(ptr as u64)
}

/// Register a `width` x `height` backing and attach it back by the handle
/// `handle_of` gets for its sid: the kernel returns this same VA.
pub fn register_backing(
    width: u32,
    height: u32,
    handle_of: impl FnOnce(i64) -> Result<u64, &'static str>,
) -> Result<Primary, &'static str> {
    let stride = width.checked_mul(4).ok_or("gop: stride overflow")?;
    let byte_len = (stride as u64).checked_mul(height as u64).ok_or("gop: byte_len overflow")?;
    let base_va = alloc_backing(byte_len)?;
    let (format, blank) = (SURFACE_FORMAT_ARGB8888, SurfaceDescriptor::default());
    let desc = SurfaceDescriptor { width, height, stride, format, byte_len, base_va, ..blank };
    let sid = mk_surface_register(&desc);
    if sid < 0 {
        super::rejected::say_once(byte_len, sid, &super::rejected::SAID, crate::say::say);
        return Err("gop: surface register rejected");
    }
    let handle = handle_of(sid)?;
    let mut attached = SurfaceDescriptor::default();
    let va = mk_surface_attach(handle, &mut attached);
    if va <= 0 || attached.format != SURFACE_FORMAT_ARGB8888 {
        return Err("gop: surface attach rejected");
    }
    Ok(Primary { handle, va: va as u64, width, height, stride, byte_len })
}
