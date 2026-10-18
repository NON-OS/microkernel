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

//! Register reads, with the bits that move on their own.

use super::sim::Sim;

impl Sim {
    pub fn read(&mut self, off: u32, n: usize) -> u32 {
        self.poll_tc();
        match off {
            0x24 => self.present() & mask(n),
            0x2f => {
                if self.srst_left > 0 {
                    self.srst_left -= 1;
                    self.regs[0x2f] as u32
                } else {
                    self.regs[0x2f] = 0;
                    0
                }
            }
            0x30 => self.int & mask(n),
            0x32 => (self.int >> 16) & mask(n),
            0x2c => {
                let mut c = self.get(0x2c, 2);
                if c & 1 != 0 {
                    c |= 2;
                }
                c & mask(n)
            }
            _ => self.get(off, n),
        }
    }
}

fn mask(n: usize) -> u32 {
    if n >= 4 {
        u32::MAX
    } else {
        (1u32 << (8 * n)) - 1
    }
}
