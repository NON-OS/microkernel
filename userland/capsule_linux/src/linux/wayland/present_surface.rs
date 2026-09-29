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

use super::scene::Scene;

/// SURFACE_FORMAT_ARGB8888 in the surface registry, which is one there
/// and zero in wl_shm. The two numbers are unrelated and both are right.
const FORMAT_ARGB8888: u32 = 1;

/// Registered once, on the first commit: its size is the first buffer's,
/// and a client does not say before then.
pub fn surface(scene: &mut Scene, width: u32, height: u32, stride: u32) -> Option<u64> {
    if let Some(handle) = scene.out {
        return Some(handle);
    }
    let desc = SurfaceDescriptor {
        width,
        height,
        stride,
        format: FORMAT_ARGB8888,
        byte_len: scene.frame_span().1,
        base_va: scene.frame_span().0,
        flags: 0,
    };
    let sid = mk_surface_register(&desc);
    let handle = if sid < 0 { sid } else { mk_surface_share(sid as u64) };
    if handle <= 0 {
        say(alloc::format!("[WAYLAND] surface {width}x{height} refused, rc {handle}\n"));
        return None;
    }
    scene.out = Some(handle as u64);
    super::present_window::place(handle as u64, width, height);
    Some(handle as u64)
}

pub(super) fn say(line: alloc::string::String) {
    let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
}
