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

//! The shape of one derivation and where each new block takes its reference
//! from, RFC 9106 section 3.4.1.2.

/// Slices per pass, the synchronization points of section 3.4.
pub(super) const SLICES: usize = 4;

pub(super) struct Shape {
    pub(super) lanes: usize,
    pub(super) lane_len: usize,
    pub(super) seg_len: usize,
    pub(super) blocks: usize,
    pub(super) passes: u32,
    /// The Argon2 type y: 0 for Argon2d, 1 for Argon2i, 2 for Argon2id.
    pub(super) y: u32,
}

/// Where a block is being made: pass, lane, slice, and index in the segment.
#[derive(Clone, Copy)]
pub(super) struct Position {
    pub(super) pass: u32,
    pub(super) lane: usize,
    pub(super) slice: usize,
    pub(super) index: usize,
}

impl Shape {
    /// Whether this segment takes its references from the address blocks
    /// rather than from the previous block.
    pub(super) fn independent(&self, at: Position) -> bool {
        self.y == 1 || (self.y == 2 && at.pass == 0 && at.slice < SLICES / 2)
    }

    /// The index in the reference lane of the block `j1` picks, where
    /// `same_lane` says whether that lane is the one being filled.
    pub(super) fn ref_index(&self, at: Position, j1: u64, same_lane: bool) -> usize {
        let done =
            if at.pass == 0 { at.slice * self.seg_len } else { self.lane_len - self.seg_len };
        let area = if same_lane {
            done + at.index - 1
        } else if at.index == 0 {
            done - 1
        } else {
            done
        };
        let x = (j1 * j1) >> 32;
        let y = (area as u64 * x) >> 32;
        let relative = area - 1 - y as usize;
        let start = match (at.pass, at.slice) {
            (0, _) => 0,
            (_, s) if s == SLICES - 1 => 0,
            (_, s) => (s + 1) * self.seg_len,
        };
        (start + relative) % self.lane_len
    }
}
