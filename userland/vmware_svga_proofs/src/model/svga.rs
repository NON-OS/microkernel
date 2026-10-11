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

//! The adapter's state.

pub struct Svga {
    pub index: u32,
    pub offers_id2: bool,
    pub id: u32,
    pub enable: u32,
    pub width: u32,
    pub height: u32,
    pub bpp: u32,
    pub max: (u32, u32),
    /// Bytes past `width * 4` on every framebuffer line.
    pub pad: u32,
    pub fb_offset: u32,
    pub vram: u32,
    pub caps: u32,
    pub mem_size: u32,
    pub mem_regs: u32,
    pub config_done: u32,
    /// The FIFO's host memory, the same words the driver maps.
    pub fifo: (u64, usize),
    pub refuses_modes: bool,
    pub stalled: bool,
    pub writes: Vec<(u32, u32)>,
    pub updates: Vec<(u32, u32, u32, u32)>,
}

impl Svga {
    /// An adapter fresh from the firmware: VGA mode, nothing configured.
    pub fn new(fifo: (u64, usize)) -> Self {
        Self {
            index: 0,
            offers_id2: true,
            id: 0x9000_0000,
            enable: 0,
            width: 640,
            height: 480,
            bpp: 8,
            max: (2560, 1600),
            pad: 0,
            fb_offset: 0,
            vram: 16 << 20,
            caps: 0,
            mem_size: fifo.1 as u32,
            mem_regs: 291,
            config_done: 0,
            fifo,
            refuses_modes: false,
            stalled: false,
            writes: Vec::new(),
            updates: Vec::new(),
        }
    }
}
