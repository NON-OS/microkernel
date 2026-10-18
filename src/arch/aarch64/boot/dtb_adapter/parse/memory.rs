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

use super::describe;
use crate::arch::aarch64::boot::info::{BootInfo, MemoryType};
use crate::arch::fdt::find::memory::{find, MemoryRange};
use crate::arch::fdt::Fdt;

/// RAM from the tree's memory nodes. A tree with none cannot be booted on:
/// the virt default would hand out addresses this board may not have.
pub(super) fn populate(fdt: &Fdt, info: &mut BootInfo) -> Result<(), &'static [u8]> {
    let mut ranges = [MemoryRange { base: 0, size: 0 }; 8];
    let count = find(fdt, &mut ranges).map_err(describe)?;
    if count == 0 {
        return Err(b"the device tree describes no memory");
    }
    info.ram_base = ranges[0].base;
    info.ram_size = ranges[0].size;
    info.memory_region_count = 0;
    for range in ranges.iter().take(count) {
        info.add_memory_region(range.base, range.size, MemoryType::Available);
    }
    Ok(())
}
