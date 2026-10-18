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

//! The transmit process: from the current descriptor while the part owns
//! it, a setup frame loaded or a frame sent whole, the descriptor given
//! back, the walk following end-of-ring; suspended at the first descriptor
//! the driver still holds.

use super::part::{Part, TS_STOPPED, TS_SUSPENDED};

const OWN: u32 = 1 << 31;
const LS: u32 = 1 << 30;
const FS: u32 = 1 << 29;
const SET: u32 = 1 << 27;
const TER: u32 = 1 << 25;

impl Part {
    pub(super) fn transmit(&mut self) {
        if self.ts == TS_STOPPED {
            return;
        }
        loop {
            let d = self.tx_at;
            if self.mem.u32(d) & OWN == 0 {
                (self.ts, self.csr[5]) = (TS_SUSPENDED, self.csr[5] | 1 << 2);
                return;
            }
            if self.faults.ignores_setup && self.mem.u32(d + 4) & SET != 0 {
                return;
            }
            let (ctl, buf) = (self.mem.u32(d + 4), self.mem.u32(d + 8));
            assert_eq!(self.mem.u32(d + 12) | ctl >> 11 & 0x7FF, 0, "buffer 2 used");
            let len = (ctl & 0x7FF) as usize;
            if ctl & SET != 0 {
                self.take_setup(buf, len);
            } else {
                assert_eq!(ctl & (FS | LS), FS | LS, "a frame chained over descriptors");
                let mut frame = vec![0u8; len];
                self.mem.read(buf, &mut frame);
                self.sent.push(frame);
            }
            self.mem.set_u32(d, 0);
            self.csr[5] |= 1;
            self.tx_at = if ctl & TER != 0 { self.csr[4] } else { d + 16 };
        }
    }
}
