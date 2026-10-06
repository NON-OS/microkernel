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

//! Match lengths: 0 to 7 and 8 to 15 per position state, 16 to 271 shared.

use super::lzma_model::HALF;
use super::range::Range;

pub struct Len {
    choice: [u16; 2],
    low: [[u16; 8]; 16],
    mid: [[u16; 8]; 16],
    high: [u16; 256],
}

impl Len {
    pub fn new() -> Len {
        Len { choice: [HALF; 2], low: [[HALF; 8]; 16], mid: [[HALF; 8]; 16], high: [HALF; 256] }
    }

    /// The length less the minimum of two.
    pub fn decode(&mut self, rc: &mut Range, pos_state: usize) -> u32 {
        if rc.bit(&mut self.choice[0]) == 0 {
            return rc.tree(&mut self.low[pos_state], 3);
        }
        if rc.bit(&mut self.choice[1]) == 0 {
            return 8 + rc.tree(&mut self.mid[pos_state], 3);
        }
        16 + rc.tree(&mut self.high, 8)
    }
}
