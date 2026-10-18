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

//! What boot keeps: the entries, and the tree's reserved-memory regions a
//! reserved range's physical span must lie in.

use spin::Once;

use super::lookup::collect;
use crate::arch::fdt::find::reserved_dma::{TreeRange, MAX_TREE_RANGES};
use crate::memory::iommu::reserved::Reserved;
use crate::memory::iommu::DeviceAddress;

/// Reserved-memory regions kept for the check.
pub(super) const MAX_REGIONS: usize = 32;

pub(super) struct Table {
    pub(super) entries: [TreeRange; MAX_TREE_RANGES],
    pub(super) count: usize,
    pub(super) regions: [(u64, u64); MAX_REGIONS],
    pub(super) region_count: usize,
}

pub(super) static TABLE: Once<Table> = Once::new();

/// `device`'s ranges; none before boot has read the tree, or without one.
pub fn for_device(device: DeviceAddress) -> Reserved {
    TABLE
        .get()
        .map_or(Reserved::NONE, |t| collect(&t.entries[..t.count.min(MAX_TREE_RANGES)], device))
}

/// The tree's reserved-memory regions, `(base, size)`.
pub fn reserved_memory() -> &'static [(u64, u64)] {
    TABLE.get().map_or(&[], |t| &t.regions[..t.region_count.min(MAX_REGIONS)])
}
