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

//! Writes to the bus master window: the cold reset seen through the link
//! control, the list address, the last valid index (which starts a halted
//! engine again), the status bits a one clears, and the control register.

use super::ich::Ich;

const DCH: u16 = 1;
const CELV: u16 = 2;
const RPBM: u8 = 1;

impl Ich {
    pub fn nabm_write(&mut self, offset: u16, width: u8, value: u32) {
        self.writes.push((super::NABM_GRANT, offset, value));
        if self.opt.decode_off {
            return;
        }
        match (offset, width) {
            (0x2C, 4) => {
                if value & 2 == 0 {
                    self.cold_low = true;
                } else if self.cold_low {
                    self.cold_cycled = true;
                }
                self.glob_cnt = value;
            }
            (0x10, 4) => self.bdbar = value & !3,
            (0x15, 1) => {
                if self.cr & RPBM != 0 && self.sr & DCH != 0 {
                    self.sr &= !(DCH | CELV);
                    self.advance();
                }
                self.lvi = value as u8 % 32;
            }
            (0x16, 2) => self.sr &= !(value as u16 & 0x1C),
            (0x1B, 1) => self.control(value as u8),
            _ => panic!("bus master write at {offset:#x}, width {width}"),
        }
    }
}
