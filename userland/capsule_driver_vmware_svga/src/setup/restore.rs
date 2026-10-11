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

//! What the firmware left in the mode registers, put back when a bring-up
//! attempt fails. The compositor may still be drawing into the firmware's
//! framebuffer; leaving the device in a mode it did not set would scramble
//! the picture it shows until the next attempt.

use crate::constants::regs::{BITS_PER_PIXEL, CONFIG_DONE, ENABLE, HEIGHT, WIDTH};
use crate::regs::Regs;

#[derive(Clone, Copy)]
pub struct Saved {
    enable: u32,
    width: u32,
    height: u32,
    bpp: u32,
    config_done: u32,
}

impl Saved {
    pub fn read(regs: &Regs) -> Self {
        Self {
            enable: regs.read(ENABLE),
            width: regs.read(WIDTH),
            height: regs.read(HEIGHT),
            bpp: regs.read(BITS_PER_PIXEL),
            config_done: regs.read(CONFIG_DONE),
        }
    }

    pub fn restore(&self, regs: &Regs) {
        regs.write(WIDTH, self.width);
        regs.write(HEIGHT, self.height);
        regs.write(BITS_PER_PIXEL, self.bpp);
        regs.write(ENABLE, self.enable);
        regs.write(CONFIG_DONE, self.config_done);
    }
}
