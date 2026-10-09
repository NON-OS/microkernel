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

use nonos_libc::{
    mk_munmap, mk_surface_register, mk_surface_share, SurfaceDescriptor, SURFACE_FORMAT_ARGB8888,
};

use super::backing::Backing;

/// Make the backing a surface the compositor can map, and the handle it is
/// shared under. The compositor is told about it later (server/scene), and
/// asked again until it answers, so a compositor busy with a frame no longer
/// sends setup back to the start. That used to map a new backing each round,
/// a whole screen's worth, and leave the old one behind.
pub fn share_surface(backing: &Backing) -> Result<u64, &'static str> {
    let desc = SurfaceDescriptor {
        width: backing.width,
        height: backing.height,
        stride: backing.stride,
        format: SURFACE_FORMAT_ARGB8888,
        byte_len: backing.byte_len,
        base_va: backing.backing_va,
        flags: 0,
    };
    let sid = mk_surface_register(&desc);
    if sid < 0 {
        give_back(backing);
        return Err("surface register rejected");
    }
    let handle = mk_surface_share(sid as u64);
    if handle <= 0 {
        // The registered surface still names the backing, so it stays.
        return Err("surface share rejected");
    }
    Ok(handle as u64)
}

/* Nothing names the backing yet: setup starts over and maps its own. */
fn give_back(backing: &Backing) {
    let _ = mk_munmap(backing.backing_va as *mut u8, backing.byte_len as usize);
}
