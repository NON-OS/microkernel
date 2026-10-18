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

//! A surface a client shared with the compositor for it to write into
//! (SCREENSHOT), attached and checked to be ARGB, or released again.

use nonos_libc::{mk_surface_attach, mk_surface_release};
use nonos_libc::{SurfaceDescriptor, SURFACE_FORMAT_ARGB8888};

use crate::sw_blitter::Surface;

pub fn attach_argb(handle: u64) -> Option<Surface> {
    let mut d = SurfaceDescriptor::default();
    let va = mk_surface_attach(handle, &mut d);
    if va <= 0 || d.format != SURFACE_FORMAT_ARGB8888 {
        let _ = (va > 0).then(|| mk_surface_release(handle));
        return None;
    }
    let (base_va, stride, width, height) = (va as u64, d.stride, d.width, d.height);
    Some(Surface { base_va, stride, width, height, byte_len: d.byte_len })
}
