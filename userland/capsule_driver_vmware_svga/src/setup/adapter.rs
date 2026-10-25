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

//! The adapter as the wire server will drive it, made from what bring-up
//! found: the registers and FIFO, the framebuffer mapping, the limits the
//! adapter reads, and the mode it set, held to the mapping.

use crate::display::{Shown, Svga};
use crate::fifo::Fifo;
use crate::init::mode::FbLayout;
use crate::regs::Regs;

/// The framebuffer mapping, `(va, len)`, and the adapter's own limits,
/// `(MAX_WIDTH x MAX_HEIGHT, VRAM_SIZE)`.
pub type Window = (u64, u64);
pub type Bounds = ((u32, u32), u32);

/// The adapter showing `(w, h)` as `layout` lays it, or why not.
pub fn adapter(
    (regs, fifo): (Regs, Fifo),
    (fb, fb_len): Window,
    (max, vram): Bounds,
    (w, h): (u32, u32),
    layout: FbLayout,
) -> Result<Svga, &'static str> {
    let mode = Shown { width: w, height: h, pitch: layout.pitch, offset: layout.offset };
    let (fb_len, generation) = (fb_len as usize, 1);
    let svga = Svga { regs, fifo, fb, fb_len, max, vram, mode, generation, fence };
    if !svga.laid_inside(mode) {
        return Err("framebuffer past VRAM");
    }
    Ok(svga)
}

fn fence() {
    // SAFETY: sfence has no operands and orders the stores before it.
    #[cfg(target_arch = "x86_64")]
    unsafe {
        core::arch::x86_64::_mm_sfence()
    };
}
