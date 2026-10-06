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

/*
 * The whole display as one overlay surface, as setup draws: a screen-sized
 * buffer shared with the compositor, each frame committed as damage over it.
 */

use nonos_app_skeleton::clients::compositor::{damage_commit, display_info, scene_submit};
use nonos_app_skeleton::PaintBuffer;
use nonos_libc::{mk_mmap, mk_surface_register, mk_surface_release, mk_surface_share};
use nonos_libc::{SurfaceDescriptor, SURFACE_FORMAT_ARGB8888};

const OVERLAY_Z: u32 = 1;
const PROT_READ_WRITE: i32 = 0x3;
const MAP_PRIVATE_ANON: i32 = 0x22;

pub(super) struct Surface {
    pub compositor: u32,
    pub router: u32,
    base: u64,
    pub width: u32,
    pub height: u32,
}

pub(super) fn open() -> Result<Surface, &'static str> {
    let (compositor, router) = super::peers::wait()?;
    let di = display_info(compositor, 1)?;
    let (width, height) = (di.width, di.height);
    let stride = width.checked_mul(4).ok_or("stride overflow")?;
    let byte_len = (stride as u64).checked_mul(height as u64).ok_or("size overflow")?;
    let base =
        mk_mmap(core::ptr::null_mut(), byte_len as usize, PROT_READ_WRITE, MAP_PRIVATE_ANON, -1, 0);
    if (base as isize) <= 0 {
        return Err("backing mmap failed");
    }
    let format = SURFACE_FORMAT_ARGB8888;
    let base_va = base as u64;
    let desc = SurfaceDescriptor { width, height, stride, format, byte_len, base_va, flags: 0 };
    let sid = mk_surface_register(&desc);
    let handle = if sid < 0 { -1 } else { mk_surface_share(sid as u64) };
    if handle <= 0 {
        return Err("surface refused");
    }
    if let Err(why) = scene_submit(compositor, 2, handle as u64, 0, 0, width, height, OVERLAY_Z) {
        let _ = mk_surface_release(handle as u64);
        return Err(why);
    }
    Ok(Surface { compositor, router, base: base_va, width, height })
}

impl Surface {
    pub(super) fn buffer(&self) -> PaintBuffer<'static> {
        let words = self.width as usize * self.height as usize;
        let pixels = unsafe { core::slice::from_raw_parts_mut(self.base as *mut u32, words) };
        PaintBuffer { pixels, stride_words: self.width, width: self.width, height: self.height }
    }

    pub(super) fn commit(&self, request_id: u32) {
        let _ = damage_commit(self.compositor, request_id, 0, 0, self.width, self.height);
    }
}
