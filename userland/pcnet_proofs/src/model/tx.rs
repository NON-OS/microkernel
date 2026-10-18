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

//! The transmit ring: every frame the driver hands over goes on the wire
//! whole, and its descriptor comes back without OWN.

use super::csr0::TXON;
use super::part::Part;
use super::wire::{buffer_len, ENP, OWN, STP, TINT};

impl Part {
    pub(super) fn transmit(&mut self) {
        for _ in 0..self.rings.tx_len {
            let d = self.rings.tdra + self.rings.tx_index as u32 * 16;
            let st = self.mem.u16(d + 6);
            if self.csr[0] & TXON == 0 || st & OWN == 0 {
                break;
            }
            assert_eq!(st & (STP | ENP), STP | ENP, "a frame chained over descriptors");
            let mut frame = vec![0u8; buffer_len(self.mem.u16(d + 4))];
            self.mem.read(self.mem.u32(d), &mut frame);
            self.sent.push(frame);
            self.mem.set_u32(d + 8, 0);
            self.mem.set_u16(d + 6, st & !OWN);
            self.rings.tx_index = (self.rings.tx_index + 1) % self.rings.tx_len;
            self.csr[0] |= TINT;
        }
    }
}
