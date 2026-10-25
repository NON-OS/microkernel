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

//! Register writes: resets, power, interrupt clears and the command.

use super::sim::Sim;

impl Sim {
    pub fn write(&mut self, off: u32, n: usize, v: u32) {
        self.poll_tc();
        match (off, n) {
            (0x2f, _) => {
                let v = v as u8;
                if v & 1 != 0 {
                    self.reset_all();
                } else if v & 6 != 0 {
                    // A CMD or DAT line reset clears the command and transfer
                    // statuses (SD Host Controller spec 2.2.28); the card
                    // insertion and removal latches (bits 6, 7) stay.
                    self.int &= 1 << 6 | 1 << 7;
                    self.tc_at = None;
                }
                self.regs[0x2f] = v;
                self.srst_left = self.cfg.reset_reads;
            }
            (0x29, _) => self.power_write(v),
            (0x30, 4) => self.int &= !v,
            (0x30, 2) => self.int &= !(v & 0xffff),
            (0x32, 2) => self.int &= !((v & 0xffff) << 16),
            (0x0c, 4) => {
                self.put(0x0c, 2, v & 0xffff);
                self.command((v >> 16) as u16);
            }
            (0x0e, 2) => self.command(v as u16),
            _ => self.put(off, n, v),
        }
    }
}
