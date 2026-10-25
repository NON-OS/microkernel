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

//! Reads of the bus master window: the link control and status, the codec
//! semaphore, and the PCM out box, as QEMU's ac97 answers them. Any other
//! offset or width panics, so a wrong register number fails the test.

use super::ich::Ich;

impl Ich {
    pub fn nabm_read(&mut self, offset: u16, width: u8) -> u32 {
        if self.opt.decode_off {
            return u32::MAX >> (32 - 8 * width as u32);
        }
        match (offset, width) {
            (0x34, 1) => {
                let held = self.cas;
                self.cas = true;
                held as u32
            }
            (0x2C, 4) => self.glob_cnt,
            (0x30, 4) => {
                if self.opt.codec && self.cold_cycled {
                    1 << 8
                } else {
                    0
                }
            }
            (0x10, 4) => self.bdbar,
            (0x14, 1) => self.civ as u32,
            (0x15, 1) => self.lvi as u32,
            (0x16, 2) => self.sr as u32,
            (0x1B, 1) => self.cr as u32,
            _ => panic!("bus master read at {offset:#x}, width {width}"),
        }
    }
}
