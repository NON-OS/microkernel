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

//! Registering the NONOS surface the pixels land on.

use nonos_libc::{mk_surface_register, mk_surface_share, SurfaceDescriptor};

use super::reshape::Shape;
use super::scene_pixels::Pixels;

/// SURFACE_FORMAT_ARGB8888 in the surface registry, which is one there
/// and zero in wl_shm. The two numbers are unrelated and both are right.
const FORMAT_ARGB8888: u32 = 1;

/// A surface over `pixels` at `shape`, shared so the compositor can map it.
/// Registered for the first buffer, and again for each buffer of another
/// shape (reshape.rs), since a registered surface keeps its size.
pub fn register(pixels: &Pixels, (width, height, stride): Shape) -> Option<u64> {
    let (base_va, byte_len) = pixels.span();
    let desc = SurfaceDescriptor {
        width,
        height,
        stride,
        format: FORMAT_ARGB8888,
        byte_len,
        base_va,
        flags: 0,
    };
    let sid = mk_surface_register(&desc);
    let handle = if sid < 0 { sid } else { mk_surface_share(sid as u64) };
    if handle <= 0 {
        say(alloc::format!("[WAYLAND] surface {width}x{height} refused, rc {handle}\n"));
        return None;
    }
    Some(handle as u64)
}

pub(super) fn say(line: alloc::string::String) {
    let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
}
