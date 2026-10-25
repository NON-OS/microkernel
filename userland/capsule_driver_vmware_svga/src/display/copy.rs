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

//! The copy of the shadow into the framebuffer at the device's pitch,
//! fenced, then SVGA_CMD_UPDATE through the FIFO so the host redraws it.

use super::svga::{Shown, Svga};
use crate::fifo::update;
use nonos_gfx_serve::{copy_rects, Shadow};
use nonos_gfx_wire::errno::ETIMEDOUT;
use nonos_gfx_wire::{Errno, Rect};

impl Svga {
    /// `rects` of `shadow` copied in and redrawn by the host. A ring the
    /// host stopped reading answers ETIMEDOUT; the pixels are in place and
    /// the next FLUSH redraws them.
    pub fn copy(&self, rects: &[Rect], shadow: &Shadow) -> Result<(), Errno> {
        let m = self.mode;
        let off = m.offset as usize;
        let (dst, len) = ((self.fb as usize + off) as *mut u8, self.frame_room() as usize - off);
        // SAFETY: the framebuffer stays mapped, frame_room bytes from fb,
        // while the driver serves; laid_inside kept offset inside it, and
        // copy_rects keeps every row inside `len`.
        unsafe {
            copy_rects(
                shadow.bytes,
                shadow.stride as usize,
                dst,
                len,
                m.pitch as usize,
                rects,
                false,
            )
        };
        (self.fence)();
        for r in rects.iter().filter(|r| inside(r, m)) {
            update(&self.regs, &self.fifo, r.x, r.y, r.width, r.height).map_err(|_| ETIMEDOUT)?;
        }
        Ok(())
    }
}

/// Whether `r` is a non-empty rectangle inside the mode: the server holds
/// rectangles to the primary, and an UPDATE outside the mode is not sent.
fn inside(r: &Rect, m: Shown) -> bool {
    let fit =
        |at: u32, len: u32, max: u32| len > 0 && at.checked_add(len).is_some_and(|e| e <= max);
    fit(r.x, r.width, m.width) && fit(r.y, r.height, m.height)
}
