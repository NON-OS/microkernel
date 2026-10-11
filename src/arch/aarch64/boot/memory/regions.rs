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

use super::PAGE_SIZE;
use crate::arch::aarch64::boot::info::{BootInfo, MemoryRegion, MemoryType};

/// The widest `Available` region the device tree reported, page aligned inward.
pub(super) fn largest_available(info: &BootInfo) -> Option<(u64, u64)> {
    info.memory_map()
        .iter()
        .filter(|r| r.region_type == MemoryType::Available)
        .filter_map(|r| bounds(r))
        .max_by_key(|(start, end)| end - start)
}

/// Page-aligned `[start, end)` for a region, or `None` if nothing is left of
/// it once both ends are aligned inward.
pub(super) fn bounds(region: &MemoryRegion) -> Option<(u64, u64)> {
    let start = region.base.checked_add(PAGE_SIZE - 1)? & !(PAGE_SIZE - 1);
    let end = region.base.checked_add(region.size)? & !(PAGE_SIZE - 1);
    (end > start).then_some((start, end))
}
