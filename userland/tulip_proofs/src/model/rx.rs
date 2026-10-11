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

//! The receive process: a frame the filter passes lands in the current
//! descriptor, FCS and all, with its length and first and last marks; with
//! no descriptor to take it the process suspends until a poll demand.

use super::part::{Part, RS_STOPPED, RS_SUSPENDED};

const OWN: u32 = 1 << 31;
const ES: u32 = 1 << 15;
const CE: u32 = 1 << 1;
const FS: u32 = 1 << 9;
const LS: u32 = 1 << 8;
const RER: u32 = 1 << 25;
pub const FCS: [u8; 4] = [0xDE, 0xAD, 0xBE, 0xEF];

#[derive(Debug, PartialEq, Eq)]
pub enum Delivered {
    Taken,
    Filtered,
    NoBuffer,
    NotRunning,
}

impl Part {
    pub fn deliver(&mut self, frame: &[u8], crc_error: bool) -> Delivered {
        if self.rs == RS_STOPPED {
            return Delivered::NotRunning;
        }
        if !self.passes(&frame[..6]) {
            return Delivered::Filtered;
        }
        let d = self.rx_at;
        if self.rs == RS_SUSPENDED || self.mem.u32(d) & OWN == 0 {
            (self.rs, self.csr[5]) = (RS_SUSPENDED, self.csr[5] | 1 << 7);
            return Delivered::NoBuffer;
        }
        let (ctl, buf) = (self.mem.u32(d + 4), self.mem.u32(d + 8));
        let data = [frame, &FCS].concat();
        let fl = (data.len() as u32) << 16;
        let status = if data.len() > (ctl & 0x7FF) as usize {
            fl | FS | ES
        } else {
            self.mem.write(buf, &data);
            fl | FS | LS | if crc_error { ES | CE } else { 0 }
        };
        self.mem.set_u32(d, status);
        self.rx_at = if ctl & RER != 0 { self.csr[3] } else { d + 16 };
        self.csr[5] |= 1 << 6;
        Delivered::Taken
    }
}
