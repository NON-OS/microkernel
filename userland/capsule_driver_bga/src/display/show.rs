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

//! Showing a primary: the mode set, which reads the mode back and puts the
//! old one back when the adapter did not take it, and the copy of the
//! shadow into video memory a FLUSH and a mode set make, fenced before
//! the answer.

use super::bga::{fits, Bga};
use crate::dispi::{current, set_mode};
use nonos_gfx_serve::{copy_rects, Shadow};
use nonos_gfx_wire::errno::EINVAL;
use nonos_gfx_wire::{Errno, Rect};

impl Bga {
    pub fn program(&mut self, w: u32, h: u32) -> Result<(), Errno> {
        if !fits(self.vram_len, w, h) {
            return Err(EINVAL);
        }
        set_mode(self.regs, w as u16, h as u16);
        match current(self.regs) {
            Some(m) if (m.width, m.height) == (w, h) => {
                self.mode = m;
                Ok(())
            }
            _ => {
                let old = self.mode;
                set_mode(self.regs, old.width as u16, old.height as u16);
                Err(EINVAL)
            }
        }
    }

    pub fn copy(&self, rects: &[Rect], shadow: &Shadow) {
        let (dst, len) = (self.vram as *mut u8, self.vram_len);
        let (src_stride, dst_stride) = (shadow.stride as usize, self.mode.stride as usize);
        // SAFETY: video memory stays mapped, `vram_len` long, while the
        // driver serves; copy_rects keeps every row inside it.
        unsafe { copy_rects(shadow.bytes, src_stride, dst, len, dst_stride, rects, false) };
        (self.fence)();
    }
}
