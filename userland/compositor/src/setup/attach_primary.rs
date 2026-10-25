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

//! Attaching a primary a driver shared, held to the shape the compositor
//! draws: ARGB8888, a row at least four bytes a pixel, every row inside the
//! mapping. A driver's word is not trusted for any of it.

use nonos_libc::{mk_surface_attach, SurfaceDescriptor, SURFACE_FORMAT_ARGB8888};

use crate::state::Primary;

pub fn attach_primary(handle: u64) -> Result<Primary, &'static str> {
    let mut desc = SurfaceDescriptor::default();
    let rc = mk_surface_attach(handle, &mut desc);
    if rc <= 0 || desc.format != SURFACE_FORMAT_ARGB8888 {
        return Err("surface attach rejected");
    }
    if desc.width == 0 || desc.height == 0 {
        return Err("surface attach geometry missing");
    }
    let min_stride = desc.width.checked_mul(4).ok_or("surface attach stride overflow")?;
    if desc.stride < min_stride {
        return Err("surface attach stride too small");
    }
    let min_bytes = (desc.stride as u64)
        .checked_mul(desc.height as u64)
        .ok_or("surface attach byte_len overflow")?;
    if desc.byte_len < min_bytes {
        return Err("surface attach byte_len too small");
    }
    let (width, height, stride, byte_len) = (desc.width, desc.height, desc.stride, desc.byte_len);
    Ok(Primary { handle, va: rc as u64, width, height, stride, byte_len })
}
