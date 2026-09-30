/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

/*
 * The whole display as one surface, above everything, the way setup draws:
 * a buffer the size of the screen, shared with the compositor and placed
 * as an overlay layer. Frames are committed as damage over all of it.
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
