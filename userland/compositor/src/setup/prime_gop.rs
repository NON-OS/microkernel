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

//! GOP-framebuffer display path. When no virtio-gpu driver announces a
//! gfx service, the compositor backs its primary surface with a
//! page-aligned buffer it owns, registers it, and presents through
//! MkSurfacePresent. The kernel blits that surface to the UEFI linear
//! framebuffer the bootloader handed off, so the desktop renders on any
//! UEFI machine: real hardware, VirtualBox, VMware, with no virtio-gpu.

use nonos_libc::{
    mk_getpid, mk_surface_attach, mk_surface_register, mk_surface_share, SurfaceDescriptor,
    SURFACE_FORMAT_ARGB8888,
};

use super::prime_gop_backing::{alloc_backing, display_dimensions};
use super::prime_gop_context::gop_context;
use crate::state::Context;

pub fn run_gop_once() -> Result<Context, &'static str> {
    let (width, height) = display_dimensions()?;
    let stride = width.checked_mul(4).ok_or("gop: stride overflow")?;
    let byte_len = (stride as u64).checked_mul(height as u64).ok_or("gop: byte_len overflow")?;

    let base_va = alloc_backing(byte_len)?;
    let desc = SurfaceDescriptor {
        width,
        height,
        stride,
        format: SURFACE_FORMAT_ARGB8888,
        byte_len,
        base_va,
        flags: 0,
        owner_pid: 0,
        writable: 0,
    };
    let sid = mk_surface_register(&desc);
    if sid < 0 {
        return Err("gop: surface register rejected");
    }
    // A sid is a slot, not a handle: shared with its own process, the kernel
    // hands back the handle and names no receiver.
    let handle = mk_surface_share(sid as u64, mk_getpid(), 0);
    if handle <= 0 {
        return Err("gop: surface handle refused");
    }
    let handle = handle as u64;

    // Self-attach: the kernel returns this same mmap VA, which has a real
    // VMA the present path resolves against. We composite into it directly.
    let mut attached = SurfaceDescriptor::default();
    let va = mk_surface_attach(handle, &mut attached);
    if va <= 0 || attached.format != SURFACE_FORMAT_ARGB8888 {
        return Err("gop: surface attach rejected");
    }

    Ok(gop_context(width, height, stride, byte_len, va as u64, handle))
}
