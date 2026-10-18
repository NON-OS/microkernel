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

//! The word accesses the driver makes, decoded: RDP at 0x10, RAP at 0x12,
//! RESET at 0x14, BDP at 0x16.

use super::events::Event;
use super::part::{Part, STOP};

impl Part {
    pub fn read16(&mut self, offset: usize) -> u16 {
        match offset {
            0x10 => match self.rap {
                88 => self.chip_id as u16,
                89 => (self.chip_id >> 16) as u16,
                n => self.csr[n as usize],
            },
            0x12 => self.rap,
            0x14 => {
                self.reset();
                0
            }
            // LED0's reset value routes link status into LEDOUT.
            0x16 if self.rap == 4 => 0x00C0 | if self.link { 0x8000 } else { 0 },
            0x16 => self.bcr[self.rap as usize],
            _ => {
                self.log.push(Event::ApromRead);
                0
            }
        }
    }

    pub fn write16(&mut self, offset: usize, v: u16) {
        match offset {
            0x10 if self.rap == 0 => self.write_csr0(v),
            0x10 if self.rap == 88 || self.rap == 89 => {}
            0x10 => self.csr[self.rap as usize] = v,
            // Seven bits select among 128 registers; the rest are reserved.
            0x12 => self.rap = v & 0x7F,
            0x16 => self.write_bcr(self.rap, v),
            _ => panic!("driver wrote {v:#x} at window offset {offset:#x}"),
        }
    }

    fn write_bcr(&mut self, n: u16, v: u16) {
        self.log.push(Event::Bcr(n, v));
        if n != 20 {
            self.bcr[n as usize] = v;
            return;
        }
        // SWSTYLE is taken only while the part is stopped. Styles 2 and 3
        // set SSIZE32 and CSRPCNET with it, as the data sheet says.
        if self.csr[0] & STOP == 0 {
            return;
        }
        let style = v & 0xFF;
        self.bcr[20] = style | if style == 2 || style == 3 { 0x0300 } else { 0 };
    }
}
