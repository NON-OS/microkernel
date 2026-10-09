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

//! Match distances: a six-bit slot per length state, then reverse-coded
//! bits for short ones, direct bits and four aligned bits for long ones.

use super::lzma_model::Model;
use super::range::Range;

const END_POS_MODEL: u32 = 14;

/// The distance less one, or None for the end marker LZMA2 never has.
pub fn distance(rc: &mut Range, m: &mut Model, len: u32) -> Option<u32> {
    let slot = rc.tree(&mut m.pos_slot[len.min(3) as usize], 6);
    if slot < 4 {
        return Some(slot);
    }
    let bits = (slot >> 1) - 1;
    let mut dist = (2 | (slot & 1)) << bits;
    if slot < END_POS_MODEL {
        let base = (dist - slot) as usize;
        dist += rc.reverse(&mut m.pos[base..], bits);
    } else {
        dist = dist.wrapping_add(rc.direct(bits - 4) << 4);
        dist = dist.wrapping_add(rc.reverse(&mut m.align, 4));
    }
    (dist != u32::MAX).then_some(dist)
}
