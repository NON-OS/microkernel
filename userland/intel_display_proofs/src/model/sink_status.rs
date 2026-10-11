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

//! What the receiver reports in 0x202-0x207, from the link it is given.

use super::sink::SinkState;
use crate::dp::lane::Drive;
use crate::dp::train::Pattern;

impl SinkState {
    fn enough(&self, need: Drive) -> bool {
        self.drive.swing >= need.swing && self.drive.pre >= need.pre
    }

    /// The bytes 0x202-0x207 the receiver reports now.
    pub fn status(&self) -> [u8; 6] {
        let Some((mbps, lanes)) = self.link else { return [0; 6] };
        let can = mbps <= self.lock_mbps && lanes <= self.lock_lanes;
        let cr = can && self.pattern != Pattern::Off && self.enough(self.cr_needs);
        let eq = cr
            && matches!(self.pattern, Pattern::Tps2 | Pattern::Tps3)
            && self.enough(self.eq_needs);
        let nib = (cr as u8) | ((eq as u8) * 0b110);
        let lane = |l: u32| if l < lanes { nib } else { 0 };
        let want = if cr { self.eq_needs } else { self.cr_needs };
        let adj = want.swing | (want.pre << 2);
        [
            lane(0) | (lane(1) << 4),
            lane(2) | (lane(3) << 4),
            eq as u8,
            0,
            adj | (adj << 4),
            adj | (adj << 4),
        ]
    }
}
