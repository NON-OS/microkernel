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

//! How big the below-4 GiB DMA pool is on a device tree boot: the broker's
//! own sizing, one page in 128 of the usable memory it could come from, which
//! the x86_64 path uses too, rather than a fixed 8 MiB on every board.

use super::PAGE_SIZE;
use crate::hardware::broker::dma::{low32_fit, low32_target_pages};

const FOUR_GIB: u64 = 1 << 32;

/// Bytes of `available` between `floor` and 4 GiB.
pub(super) fn low_usable(available: &[(u64, u64)], floor: u64) -> u64 {
    let clip = |(start, end): &(u64, u64)| {
        let (lo, hi) = ((*start).max(floor), (*end).min(FOUR_GIB));
        hi.saturating_sub(lo)
    };
    available.iter().map(clip).sum()
}

/// The sizes to try, largest first: the target, then halves of it while a
/// pool that size is still worth reserving.
pub(super) fn sizes(low_usable: u64) -> impl Iterator<Item = u64> {
    let target = low32_target_pages(low_usable);
    core::iter::successors(Some(target), |pages| Some(pages / 2))
        .map(move |pages| low32_fit(target, pages))
        .take_while(|pages| *pages != 0)
        .map(|pages| pages as u64 * PAGE_SIZE)
}
