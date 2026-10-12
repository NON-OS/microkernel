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

//! The adapter as the wire server drives it: the DISPI registers, video
//! memory mapped write-combining, what the monitor's EDID said, and the
//! mode it shows now.

use crate::dispi::DispiMode;
use crate::edid::Panel;
use crate::regs::Regs;

/// The largest side this driver sets; video memory bounds it first on
/// every machine seen (QEMU's own DISPI limit is higher; unverified).
pub const MAX_SIDE: u32 = 4096;
/// DISPI widths are kept to multiples of 8 (unverified against vga.c,
/// and every common mode is one).
pub const WIDTH_ALIGN: u16 = 8;

pub struct Bga {
    pub regs: Regs,
    pub vram: u64,
    pub vram_len: usize,
    pub panel: Panel,
    pub mode: DispiMode,
    /// Orders the copy's stores before the answer: an sfence on x86.
    pub fence: fn(),
}

/// Whether a `w` x `h` mode is one this driver sets in `vram_len` bytes
/// of video memory: section 12's range from 640x480, aligned, and fitting.
pub fn fits(vram_len: usize, w: u32, h: u32) -> bool {
    let sides = (640..=MAX_SIDE).contains(&w) && (480..=MAX_SIDE).contains(&h);
    sides && w.is_multiple_of(WIDTH_ALIGN as u32) && w as usize * h as usize * 4 <= vram_len
}
