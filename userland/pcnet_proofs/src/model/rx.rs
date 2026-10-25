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

//! The receive ring: a frame lands in the next descriptor the driver gave
//! back, FCS and all, or counts as missed when there is none.

use super::csr0::RXON;
use super::part::Part;
use super::wire::{buffer_len, Delivered, BUFF, CRC, ENP, ERR, FCS, MISS, OWN, RINT, STP};

impl Part {
    /// One frame off the wire into the next receive descriptor.
    pub fn deliver(&mut self, frame: &[u8], crc_error: bool) -> Delivered {
        if self.csr[0] & RXON == 0 {
            return Delivered::NotRunning;
        }
        let d = self.rings.rdra + self.rings.rx_index as u32 * 16;
        if self.mem.u16(d + 6) & OWN == 0 {
            self.csr[0] |= MISS;
            return Delivered::NoBuffer;
        }
        let data = [frame, &FCS].concat();
        let status = if data.len() > buffer_len(self.mem.u16(d + 4)) {
            ERR | BUFF | STP
        } else {
            self.mem.write(self.mem.u32(d), &data);
            self.mem.set_u32(d + 8, data.len() as u32);
            if crc_error {
                ERR | CRC | STP | ENP
            } else {
                STP | ENP
            }
        };
        self.mem.set_u16(d + 6, status);
        self.rings.rx_index = (self.rings.rx_index + 1) % self.rings.rx_len;
        self.csr[0] |= RINT;
        Delivered::Taken
    }
}
