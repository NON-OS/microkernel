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

//! The receive queue: a frame lands in the next ring 0 descriptor the
//! driver handed over, and a completion says where and how long.

use super::device::Device;

#[derive(Debug, PartialEq, Eq)]
pub enum Delivered {
    Taken,
    NoBuffer,
    NotRunning,
}

impl Device {
    pub fn deliver(&mut self, frame: &[u8], err: bool) -> Delivered {
        if !self.active {
            return Delivered::NotRunning;
        }
        let d = self.rx.at(self.rx.next);
        let d2 = self.mem.u32(d + 8);
        if d2 >> 31 != self.rx.gen {
            return Delivered::NoBuffer;
        }
        assert_eq!((d2 >> 14) & 3, 0, "only head buffers, descriptor type 0");
        let cap = (d2 & 0x3FFF) as usize;
        let (len, bad) = if frame.len() > cap {
            (cap, true)
        } else {
            self.mem.write(self.mem.u64(d), frame);
            (frame.len(), err)
        };
        self.complete(self.rx.next, 0, len as u32 | (bad as u32) << 14);
        self.rx.advance();
        Delivered::Taken
    }

    /// A whole-frame completion for slot `idx` of ring `rq`, as the device
    /// writes one; a test calls it directly to forge a stray one.
    pub fn complete(&mut self, idx: u32, rq: u32, d2: u32) {
        let c = self.rx_comp.at(self.rx_comp.next);
        self.mem.set_u32(c, idx | 1 << 14 | 1 << 15 | rq << 16);
        self.mem.set_u32(c + 4, 0);
        self.mem.set_u32(c + 8, d2);
        self.mem.set_u32(c + 12, self.rx_comp.gen << 31 | 3 << 24);
        self.rx_comp.advance();
    }
}
