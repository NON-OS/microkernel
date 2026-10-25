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

//! The codec's register numbers, by register over two, and its state out of
//! a reset: QEMU's STAC9700, outputs muted, variable rate on at 22.05 kHz so
//! a driver that leaves it on plays at the wrong rate.

use super::ich::Ich;

pub const MASTER: usize = 0x02 >> 1;
pub const HEADPHONE: usize = 0x04 >> 1;
pub const PCM_OUT: usize = 0x18 >> 1;
pub const POWERDOWN: usize = 0x26 >> 1;
pub const EXT_CTRL: usize = 0x2A >> 1;
pub const FRONT_RATE: usize = 0x2C >> 1;
pub const VENDOR1: usize = 0x7C >> 1;
pub const VENDOR2: usize = 0x7E >> 1;

impl Ich {
    pub fn reset_codec(&mut self) {
        self.mixer.iter_mut().for_each(|r| *r = 0);
        self.mixer[MASTER] = 0x8000;
        self.mixer[HEADPHONE] = 0x8000;
        self.mixer[PCM_OUT] = 0x8808;
        self.mixer[VENDOR1] = 0x8384;
        self.mixer[VENDOR2] = 0x7600;
        self.mixer[0x28 >> 1] = 0x0809;
        self.mixer[EXT_CTRL] = 0x0009;
        self.mixer[FRONT_RATE] = 22_050;
    }
}
