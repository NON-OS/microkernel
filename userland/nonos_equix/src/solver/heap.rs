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


//! Slot arithmetic and the packed pair records. A record names the left
//! bucket in its low 8 bits, the right item's position in the next 9 and the
//! left item's position above that, as the reference packs it.

use super::{COARSE_BUCKETS, COARSE_ITEMS, FINE_BUCKETS};

pub fn slot(bucket: usize, pos: usize) -> usize {
    bucket * COARSE_ITEMS + pos
}

pub fn make_item(bucket: usize, left: usize, right: usize) -> u32 {
    (left as u32) << 17 | (right as u32) << 8 | bucket as u32
}

pub fn item_bucket(item: u32) -> usize {
    (item % COARSE_BUCKETS as u32) as usize
}

pub fn item_left(item: u32) -> usize {
    (item >> 17) as usize
}

pub fn item_right(item: u32) -> usize {
    ((item >> 8) & 511) as usize
}

/// The bucket whose values complete this one's to a multiple of 256.
pub fn invert_bucket(bucket: usize) -> usize {
    ((bucket as u32).wrapping_neg() % COARSE_BUCKETS as u32) as usize
}

pub fn invert_scratch(bucket: usize) -> usize {
    ((bucket as u32).wrapping_neg() % FINE_BUCKETS as u32) as usize
}
