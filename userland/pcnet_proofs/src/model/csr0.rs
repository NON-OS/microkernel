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

//! CSR0: the causes clear when written as one, STOP stops, INIT reads the
//! init block, STRT starts both engines, TDMD sends now.

use super::events::Event;
use super::part::{Part, STOP};

pub const INIT: u16 = 1 << 0;
pub const STRT: u16 = 1 << 1;
pub const TDMD: u16 = 1 << 3;
pub const TXON: u16 = 1 << 4;
pub const RXON: u16 = 1 << 5;
pub const IDON: u16 = 1 << 8;

impl Part {
    pub(super) fn write_csr0(&mut self, v: u16) {
        self.log.push(Event::Csr0(v));
        self.csr[0] &= !(v & 0x7F00);
        if v & STOP != 0 {
            self.csr[0] = STOP;
            self.ready = false;
            return;
        }
        if v & INIT != 0 && !self.faults.deaf_to_init {
            self.init();
        }
        if v & STRT != 0 && self.ready && !self.faults.never_starts {
            self.csr[0] = (self.csr[0] & !STOP) | STRT | RXON | TXON;
        }
        if v & TDMD != 0 && self.csr[0] & TXON != 0 {
            self.transmit();
        }
    }

    // Only the 32-bit block is read: in any other style the part would take
    // a 24-byte layout with 24-bit addresses out of the same bytes.
    fn init(&mut self) {
        if self.bcr[20] & 0x01FF != 0x0102 {
            return;
        }
        let iadr = self.csr[1] as u32 | (self.csr[2] as u32) << 16;
        let mut b = [0u8; 28];
        self.mem.read(iadr, &mut b);
        let r = &mut self.rings;
        r.mode = u16::from_le_bytes([b[0], b[1]]);
        r.rx_len = 1 << (b[2] >> 4).min(9);
        r.tx_len = 1 << (b[3] >> 4).min(9);
        r.padr.copy_from_slice(&b[4..10]);
        r.ladrf.copy_from_slice(&b[12..20]);
        r.rdra = u32::from_le_bytes([b[20], b[21], b[22], b[23]]);
        r.tdra = u32::from_le_bytes([b[24], b[25], b[26], b[27]]);
        (r.rx_index, r.tx_index) = (0, 0);
        self.ready = true;
        self.csr[0] = (self.csr[0] & !STOP) | INIT | IDON;
        self.log.push(Event::Init);
    }
}
