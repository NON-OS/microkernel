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

//! The lane status bytes a sink reports during training, 0x202 to 0x207.

use super::dpcd::{CHANNEL_EQ_DONE, CR_DONE, INTERLANE_ALIGN_DONE, SYMBOL_LOCKED};
use super::lane::Drive;

/// The six bytes 0x202-0x207: lane status, alignment, adjust requests.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LaneStatus(pub [u8; 6]);

impl LaneStatus {
    fn lane(&self, l: u32) -> u8 {
        (self.0[(l / 2) as usize] >> (4 * (l % 2))) & 0xF
    }

    pub fn cr_done(&self, lanes: u32) -> bool {
        (0..lanes).all(|l| self.lane(l) & CR_DONE != 0)
    }

    pub fn eq_done(&self, lanes: u32) -> bool {
        let all = CR_DONE | CHANNEL_EQ_DONE | SYMBOL_LOCKED;
        let aligned = self.0[2] & INTERLANE_ALIGN_DONE != 0;
        aligned && (0..lanes).all(|l| self.lane(l) & all == all)
    }

    /// The drive the sink asks for: per lane, swing in the low two bits
    /// of its nibble at 0x206-0x207 and pre-emphasis in the next two. The
    /// transmitter sets one level for every lane, so the highest asked.
    pub fn requested(&self, lanes: u32) -> Drive {
        let mut d = Drive::default();
        for l in 0..lanes {
            let n = (self.0[4 + (l / 2) as usize] >> (4 * (l % 2))) & 0xF;
            d.swing = d.swing.max(n & 3);
            d.pre = d.pre.max(n >> 2);
        }
        d.legal()
    }
}
