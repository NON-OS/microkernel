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

//! The RAM QEMU `virt` gives a guest that boots without a device tree.

use super::limits::MAX_MEMORY_REGIONS;
use super::memory::{MemoryRegion, MemoryType};

pub(super) const RAM_BASE: u64 = 0x4000_0000;
/// Deliberately small. Without a device tree there is no way to learn how
/// much memory a board has, and the two errors are not symmetric: claiming
/// less than exists wastes it, while claiming more hands the frame allocator
/// addresses that answer to nothing. A device tree replaces this.
pub(super) const RAM_SIZE: u64 = 0x2000_0000;

/// The one-entry map the scalars above describe, built from the same figures
/// so the map and the fields beside it cannot disagree.
pub(super) const fn memory_map() -> [MemoryRegion; MAX_MEMORY_REGIONS] {
    let mut regions = [MemoryRegion::EMPTY; MAX_MEMORY_REGIONS];
    regions[0] =
        MemoryRegion { base: RAM_BASE, size: RAM_SIZE, region_type: MemoryType::Available };
    regions
}
