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

//! How full the low pool is, and the part of it only a 32-bit device may
//! take. Free of kernel state so the host proofs include it. How big it is
//! made at boot is the arch's (`arch::common::low32_sizing`).

/// The most pages the pool's bitmap tracks.
pub(crate) const LOW32_MAX_PAGES_TRACKED: usize = 8192;

/// The floor only maps for a 32-bit device (`DMA_MAP_DMA32`) may take: half
/// the pool, as the fixed 1,024 of 2,048 pages was. Other maps stop short of
/// it, so drivers that do not need the pool (xHCI, HDA, NVMe and the like
/// spawn first) cannot leave nothing for one that does.
pub(crate) const fn low32_floor(pool_pages: usize) -> usize {
    pool_pages / 2
}

/// Pressure in quarters: 0 under a quarter used, 4 when full.
pub(crate) const fn pressure_quarter(used: usize, pool_pages: usize) -> usize {
    if pool_pages == 0 {
        return 0;
    }
    let q = used.saturating_mul(4) / pool_pages;
    if q > 4 {
        4
    } else {
        q
    }
}
