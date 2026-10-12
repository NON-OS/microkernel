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

//! Sharing the display data buffer (SKL PRM Vol 12, display buffer
//! allocation): 896 blocks of 512 bytes, split evenly between the lit
//! pipes; within a pipe the cursor takes a fixed share (32 blocks with one
//! pipe lit, 8 with more) and the primary plane the rest. Sizes are the
//! PRM's as this driver reads them, unverified against the volume.

pub const BLOCKS: u32 = 896;

/// A run of blocks, end exclusive.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Run {
    pub start: u32,
    pub end: u32,
}

impl Run {
    pub fn len(&self) -> u32 {
        self.end - self.start
    }

    pub fn is_empty(&self) -> bool {
        self.end == self.start
    }

    /// PLANE_BUF_CFG and CUR_BUF_CFG: end block (inclusive) in 25:16,
    /// start in 9:0; an empty run is written as all zero.
    pub fn cfg(&self) -> u32 {
        if self.is_empty() {
            return 0;
        }
        ((self.end - 1) << 16) | self.start
    }
}

/// The (plane, cursor) runs of each pipe in `lit`, pipe A first; a dark
/// pipe gets empty runs.
pub fn allocate(lit: [bool; 3]) -> [(Run, Run); 3] {
    let n = lit.iter().filter(|&&l| l).count() as u32;
    let mut out = [(Run::default(), Run::default()); 3];
    if n == 0 {
        return out;
    }
    let share = BLOCKS / n;
    let cursor = if n == 1 { 32 } else { 8 };
    let mut at = 0;
    for (i, _) in lit.iter().enumerate().filter(|(_, &l)| l) {
        let plane = Run { start: at, end: at + share - cursor };
        out[i] = (plane, Run { start: plane.end, end: at + share });
        at += share;
    }
    out
}
