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

//! Reading the mode the firmware left, and setting one. A mode takes effect
//! with ENABLE, and the device answers with the line pitch and where in VRAM
//! the visible framebuffer starts.

use crate::constants::mode::{BITS_PER_PIXEL as BPP, BYTES_PER_PIXEL};
use crate::constants::regs::{BITS_PER_PIXEL, BYTES_PER_LINE, ENABLE, FB_OFFSET, HEIGHT, WIDTH};
use crate::regs::Regs;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FbLayout {
    pub pitch: u32,
    pub offset: u32,
}

/// The 32-bit mode the firmware enabled, if it enabled one.
pub fn firmware_mode(regs: &Regs) -> Option<(u32, u32)> {
    if regs.read(ENABLE) == 0 || regs.read(BITS_PER_PIXEL) != BPP {
        return None;
    }
    Some((regs.read(WIDTH), regs.read(HEIGHT)))
}

pub fn set_mode(regs: &Regs, (w, h): (u32, u32)) -> Result<FbLayout, &'static str> {
    regs.write(WIDTH, w);
    regs.write(HEIGHT, h);
    regs.write(BITS_PER_PIXEL, BPP);
    regs.write(ENABLE, 1);
    if regs.read(WIDTH) != w || regs.read(HEIGHT) != h || regs.read(ENABLE) == 0 {
        return Err("mode not taken");
    }
    let pitch = regs.read(BYTES_PER_LINE);
    if pitch < w * BYTES_PER_PIXEL {
        return Err("line pitch shorter than a line");
    }
    Ok(FbLayout { pitch, offset: regs.read(FB_OFFSET) })
}
