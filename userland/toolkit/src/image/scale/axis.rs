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

use alloc::vec::Vec;

use super::{area, linear};

// Fixed-point unit of one tap weight; every destination's weights sum to it.
pub const UNIT: u32 = 1 << 12;

// Filter taps along one axis: destination `i` reads `taps` source cells
// starting at `start[i]`, weighted by `weight[i * taps ..]`.
pub struct Axis {
    pub start: Vec<u32>,
    pub weight: Vec<u16>,
    pub taps: usize,
}

// `off` and `span` are the source window in 16.16 pixels, inside `src`.
pub fn axis(src: u32, off: u64, span: u64, dst: u32) -> Option<Axis> {
    if src == 0 || dst == 0 || span == 0 || off + span > (src as u64) << 16 {
        return None;
    }
    if span > (dst as u64) << 16 {
        // Shrinking: average every source cell a destination pixel covers.
        let taps = (span / dst as u64 >> 16) as usize + 2;
        Some(area::area(src, off, span, dst, taps))
    } else {
        // Enlarging or 1:1: interpolate between the two nearest centres.
        Some(linear::linear(src, off, span, dst))
    }
}
