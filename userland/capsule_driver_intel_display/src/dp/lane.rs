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

//! Drive levels and lane status (DisplayPort 1.4a section 3.1.5 and the
//! DPCD's training and status fields). Levels run 0 to 3 for swing and
//! for pre-emphasis, and a swing level plus a pre-emphasis level may not
//! pass 3.

pub use super::status::LaneStatus;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Drive {
    pub swing: u8,
    pub pre: u8,
}

impl Drive {
    /// The nearest level the standard allows: each at most 3, together
    /// at most 3, pre-emphasis yielding to swing.
    pub fn legal(self) -> Drive {
        let swing = self.swing.min(3);
        Drive { swing, pre: self.pre.min(3 - swing) }
    }

    /// TRAINING_LANEx_SET: swing in 1:0, max swing reached 2, pre-emphasis
    /// in 4:3, max pre-emphasis reached 5.
    pub fn lane_set(self) -> u8 {
        let d = self.legal();
        let max_swing = (d.swing == 3) as u8;
        let max_pre = (d.swing + d.pre == 3) as u8;
        d.swing | (max_swing << 2) | (d.pre << 3) | (max_pre << 5)
    }
}
