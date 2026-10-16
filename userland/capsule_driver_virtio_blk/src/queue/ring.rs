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

//! The split ring's layout in its grant, the legacy one for both
//! transports: descriptors from the start, the available ring after `size`
//! of them, the used ring from the next page. A legacy device finds the
//! parts from the size it dictates and one page number; a modern device is
//! told each part's address.

use crate::constants::{MIN_QUEUE_SIZE, VQ_DESC_OFFSET};

const DESC_BYTES: usize = 16;
const PAGE: usize = 4096;
/// Each ring's flags, index and event word, two bytes each.
const RING_WORDS: usize = 6;

const fn page_up(v: usize) -> usize {
    (v + PAGE - 1) & !(PAGE - 1)
}

pub const fn avail_offset(size: u16) -> usize {
    VQ_DESC_OFFSET + size as usize * DESC_BYTES
}

pub const fn used_offset(size: u16) -> usize {
    page_up(avail_offset(size) + RING_WORDS + 2 * size as usize)
}

/// The bytes a ring of `size` entries takes, in whole pages. `None` for a
/// size the driver does not lay out: one the split ring does not allow
/// (not a power of two), or one too small for a request's three
/// descriptors. Every power of two up to the specification's 32768 is laid
/// out, the grant sized to match (860160 bytes at the most).
pub fn ring_bytes(size: u16) -> Option<usize> {
    if !size.is_power_of_two() || size < MIN_QUEUE_SIZE {
        return None;
    }
    Some(page_up(used_offset(size) + RING_WORDS + 8 * size as usize))
}

/// The ring a modern device is told: the largest power of two it offers,
/// up to `cap`. `None` when the device offers none (zero) or too small one.
pub fn fit(offered: u16, cap: u16) -> Option<u16> {
    let size = 1u16 << core::cmp::min(offered, cap).checked_ilog2()?;
    ring_bytes(size).map(|_| size)
}
