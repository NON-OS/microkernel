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

//! The transmit queue on TXPROD: every descriptor whose generation is the
//! ring's goes out whole, and one asking for completion gets one.

use super::device::Device;

const EOP: u32 = 1 << 12;
const CQ: u32 = 1 << 13;

impl Device {
    pub(super) fn transmit(&mut self) {
        if !self.active {
            return;
        }
        for _ in 0..self.tx.size {
            let d = self.tx.at(self.tx.next);
            let d2 = self.mem.u32(d + 8);
            if (d2 >> 14) & 1 != self.tx.gen {
                break;
            }
            let d3 = self.mem.u32(d + 12);
            assert!(d3 & EOP != 0, "a frame chained over descriptors");
            let len = match d2 & 0x3FFF {
                0 => 16384,
                n => n as usize,
            };
            let mut frame = vec![0u8; len];
            self.mem.read(self.mem.u64(d), &mut frame);
            self.sent.push(frame);
            if d3 & CQ != 0 {
                let c = self.tx_comp.at(self.tx_comp.next);
                self.mem.write(c, &[0u8; 12]);
                self.mem.set_u32(c, self.tx.next);
                self.mem.set_u32(c + 12, self.tx_comp.gen << 31);
                self.tx_comp.advance();
            }
            self.tx.advance();
        }
    }
}
