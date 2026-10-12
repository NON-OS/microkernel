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

//! The kernel's boot tables, written before translation is on: RAM identity
//! mapped and direct mapped, then the registers boot needs.

use super::super::boot::info::{BootInfo, MemoryRegion, MemoryType};
use super::direct_map::map_direct_range;
use super::geometry::DIRECT_L0_INDEX;
use super::ram_map::map_range;
use super::state;
use crate::sys::serial;

/// Fill the tables `state::l0` roots.
///
/// # Safety
///
/// Boot CPU, once, before these tables are live.
pub(super) unsafe fn build(boot_info: &BootInfo) {
    state::l0().set_table(0, state::l1_addr());
    // The direct map hangs off its own level 1, since TTBR1 walks this same
    // level 0 and the two windows differ by more than the top bits.
    state::l0().set_table(DIRECT_L0_INDEX, state::l1_high_addr());
    let fallback = [MemoryRegion {
        base: boot_info.ram_base,
        size: boot_info.ram_size,
        region_type: MemoryType::Available,
    }];
    let ram = match boot_info.memory_map() {
        [] => &fallback[..],
        map => map,
    };
    for region in ram {
        if map_range(region.base, region.size, region.region_type).is_err() {
            serial::println(b"[MMU] out of level 2 tables: RAM past here is not identity mapped");
        }
        map_direct_range(region.base, region.size);
    }
    super::boot_devices::map_devices(boot_info);
}
