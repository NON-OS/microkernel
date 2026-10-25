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

//! The registers by number, as svga_reg.h numbers them and QEMU answers.

use super::svga::Svga;

const SVGA_ID_2: u32 = 0x9000_0002;

impl Svga {
    pub fn read(&mut self, reg: u32) -> u32 {
        match reg {
            0 => self.id,
            1 => self.enable,
            2 => self.width,
            3 => self.height,
            4 => self.max.0,
            5 => self.max.1,
            7 => self.bpp,
            12 => self.width * 4 + self.pad,
            14 => self.fb_offset,
            15 => self.vram,
            17 => self.caps,
            19 => self.mem_size,
            20 => self.config_done,
            30 => self.mem_regs,
            _ => panic!("driver read register {reg}"),
        }
    }

    pub fn write(&mut self, reg: u32, v: u32) {
        self.writes.push((reg, v));
        match reg {
            0 if v == SVGA_ID_2 && self.offers_id2 => self.id = v,
            0 => {}
            1 => self.enable = v,
            2 if !self.refuses_modes && v <= self.max.0 => self.width = v,
            3 if !self.refuses_modes && v <= self.max.1 => self.height = v,
            2 | 3 => {}
            7 => self.bpp = v,
            20 => self.config_done = v,
            21 if !self.stalled => self.run_fifo(),
            21 => {}
            _ => panic!("driver wrote {v:#x} to register {reg}"),
        }
    }
}
