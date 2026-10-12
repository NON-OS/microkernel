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
use nonos_gfx_serve::Region;
use nonos_gfx_wire::errno::EINVAL;
use nonos_gfx_wire::Errno;
use nonos_libc::{
    mk_surface_register, mk_surface_release, mk_surface_share, SurfaceDescriptor,
    SURFACE_FORMAT_ARGB8888,
};

/// Registers and shares in one step: the wire hands out the share handle,
/// never the private surface id.
pub fn publish(r: Region, width: u32, height: u32, stride: u32) -> Result<u64, Errno> {
    let desc = SurfaceDescriptor {
        width,
        height,
        stride,
        format: SURFACE_FORMAT_ARGB8888,
        byte_len: r.len as u64,
        base_va: r.base as u64,
        flags: 0,
    };
    let sid = mk_surface_register(&desc);
    let handle = if sid < 0 { sid } else { mk_surface_share(sid as u64) };
    if handle <= 0 {
        return Err(EINVAL);
    }
    Ok(handle as u64)
}

pub fn withdraw(handle: u64) {
    let _ = mk_surface_release(handle);
}
