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

//! Where free RAM is. With a NONOS loader, its EFI map says, since the
//! device tree calls RAM available that the loader's handoff, its memory
//! map and the firmware's runtime services still hold; without one, the
//! tree's Available regions, as before.

use super::clip::within;
use super::efi_runs::from_efi;
use super::loader_map;
use super::regions::bounds;
use super::usable::Usable;
use crate::arch::aarch64::boot::info::{BootInfo, MemoryType};
use crate::arch::aarch64::boot::loader::taken;

pub(super) struct Ram {
    pub(super) usable: Usable,
    /// From the loader's map: only what it names free may be handed out.
    pub(super) from_loader: bool,
}

pub(super) fn ram(info: &BootInfo) -> Result<Ram, &'static [u8]> {
    let Some(handoff) = taken() else {
        let mut usable = Usable::EMPTY;
        let tree = info.memory_map().iter().filter(|r| r.region_type == MemoryType::Available);
        for (start, end) in tree.filter_map(bounds) {
            usable.add(start, end);
        }
        return Ok(Ram { usable: reachable(info, &usable), from_loader: false });
    };
    let entries = loader_map::entries(info, handoff)?;
    let map = entries.iter().map(|e| (e.memory_type, e.physical_start, e.page_count, e.attribute));
    let usable = reachable(info, &from_efi(map));
    if usable.runs().is_empty() {
        return Err(b"the loader's memory map names no free RAM");
    }
    Ok(Ram { usable, from_loader: true })
}

/// `free` held to the tree's RAM, which the direct map covers, and below
/// where the direct map stops.
fn reachable(info: &BootInfo, free: &Usable) -> Usable {
    let ram = info.memory_map().iter().filter_map(|r| Some((r.base, r.base.checked_add(r.size)?)));
    within(free, ram, crate::arch::aarch64::mmu::direct_map_end())
}
