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

//! The primary surface the compositor draws into: the capsule's own memory,
//! registered with the kernel and shared with the compositor alone at its
//! get_primary (K20). The framebuffer itself is never handed out; a device
//! page in another process's map would outlive the broker's revocation of
//! this capsule's grant.

use nonos_libc::{
    mk_mmap, mk_munmap, mk_surface_register, SurfaceDescriptor, SURFACE_FORMAT_ARGB8888,
};

use crate::constants::mode::BYTES_PER_PIXEL;
use crate::screen::Plane;

const PROT_READ_WRITE: i32 = 0x1 | 0x2;
const MAP_PRIVATE_ANON: i32 = 0x02 | 0x20;

/// The surface's plane and the id the kernel registered it under. The
/// driver draws through its own mapping and takes no handle of its own.
pub fn create(width: u32, height: u32) -> Result<(Plane, u64), &'static str> {
    let stride = width * BYTES_PER_PIXEL;
    let bytes = (stride as u64 * height as u64 + 0xFFF) & !0xFFF;
    let base =
        mk_mmap(core::ptr::null_mut(), bytes as usize, PROT_READ_WRITE, MAP_PRIVATE_ANON, -1, 0);
    if base.is_null() || (base as i64) < 0 {
        return Err("surface memory refused");
    }
    let desc = SurfaceDescriptor {
        width,
        height,
        stride,
        format: SURFACE_FORMAT_ARGB8888,
        byte_len: bytes,
        base_va: base as u64,
        ..Default::default()
    };
    let sid = mk_surface_register(&desc);
    if sid < 0 {
        let _ = mk_munmap(base, bytes as usize);
        return Err("surface register refused");
    }
    Ok((Plane { va: base as u64, stride, width, height }, sid as u64))
}
