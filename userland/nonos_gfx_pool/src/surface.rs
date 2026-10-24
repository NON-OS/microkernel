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
use nonos_libc::surface_registry::SURFACE_SHARE_WRITE;
use nonos_libc::{
    mk_getpid, mk_surface_register, mk_surface_release, mk_surface_share, SurfaceDescriptor,
    SURFACE_FORMAT_ARGB8888,
};

use super::receiver::compositor_pid;

/// Registers and shares in one step, answering (handle, sid): the wire hands
/// out the handle, the pool keeps the sid to name a later receiver. The share
/// names its receiver (K20): the compositor, the one caller the kernel lets
/// reach a display driver, which draws into it and so maps it writable.
pub fn publish(r: Region, width: u32, height: u32, stride: u32) -> Result<(u64, u64), Errno> {
    let to = compositor_pid().ok_or(EINVAL)?;
    let desc = SurfaceDescriptor {
        width,
        height,
        stride,
        format: SURFACE_FORMAT_ARGB8888,
        byte_len: r.len as u64,
        base_va: r.base as u64,
        flags: 0,
        owner_pid: 0,
        writable: 0,
    };
    let sid = mk_surface_register(&desc);
    if sid < 0 {
        return Err(EINVAL);
    }
    let handle = mk_surface_share(sid as u64, to, SURFACE_SHARE_WRITE);
    if handle <= 0 {
        withdraw_unshared(sid as u64);
        return Err(EINVAL);
    }
    Ok((handle as u64, sid as u64))
}

/// A surface registered but never shared: the kernel names its frames until
/// the owner lets go, and the server frees the pages on this refusal, so it
/// lets go now. Register hands back no handle, only the sid; a share to the
/// owner's own process answers the owner's handle and records no right.
fn withdraw_unshared(sid: u64) {
    let own = mk_surface_share(sid, mk_getpid(), 0);
    if own > 0 {
        withdraw(own as u64);
    }
}

pub fn withdraw(handle: u64) {
    let _ = mk_surface_release(handle);
}
