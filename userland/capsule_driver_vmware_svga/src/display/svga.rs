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

//! The adapter's live state: its registers and FIFO, the framebuffer BAR
//! as mapped, its own limits, and the mode it shows now.

use crate::constants::mode::BYTES_PER_PIXEL;
use crate::fifo::Fifo;
use crate::regs::Regs;

/// The mode on screen: its size, the line pitch the device reported and
/// where in VRAM the visible framebuffer starts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Shown {
    pub width: u32,
    pub height: u32,
    pub pitch: u32,
    pub offset: u32,
}

pub struct Svga {
    pub regs: Regs,
    pub fifo: Fifo,
    /// The framebuffer BAR mapped from its first byte, `fb_len` bytes.
    pub fb: u64,
    pub fb_len: usize,
    /// MAX_WIDTH and MAX_HEIGHT, and VRAM_SIZE, as the adapter reads them.
    pub max: (u32, u32),
    pub vram: u32,
    pub mode: Shown,
    /// Moves when the output was lost and set again (STATE).
    pub generation: u32,
    /// Orders the copy's stores before the host is asked to redraw.
    pub fence: fn(),
}

impl Svga {
    /// The memory a frame may use: VRAM, and no more than is mapped.
    pub fn frame_room(&self) -> u64 {
        (self.vram as u64).min(self.fb_len as u64)
    }

    /// Whether a `w` x `h` mode is one the adapter can show from this
    /// mapping: inside its maximum, and its packed frame inside VRAM. The
    /// pitch the device then reports is held to the mapping again.
    pub fn fits(&self, w: u32, h: u32) -> bool {
        let bytes = w as u64 * h as u64 * BYTES_PER_PIXEL as u64;
        w > 0 && h > 0 && w <= self.max.0 && h <= self.max.1 && bytes <= self.frame_room()
    }

    /// Whether the device's layout for `m` stays inside the mapping.
    pub fn laid_inside(&self, m: Shown) -> bool {
        let end = (m.offset as u64).checked_add(m.pitch as u64 * m.height as u64);
        m.pitch >= m.width * BYTES_PER_PIXEL && end.is_some_and(|e| e <= self.frame_room())
    }
}
