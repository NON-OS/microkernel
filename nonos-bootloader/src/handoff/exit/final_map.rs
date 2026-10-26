// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! The firmware's last memory map, copied into the handoff once boot
//! services are gone, and handed back as the entries the kernel will read.

use uefi::table::boot::MemoryMap as UefiMemoryMap;

use crate::handoff::jump::{copy_memory_map, finalize_mmap, MemoryMapEntry};
use crate::handoff::prepare::HandoffAllocations;
use crate::handoff::types::BootHandoffV1;

/// The halt code for a memory map that outgrew its buffer after boot
/// services were gone, the same as x86_64's.
const MAP_OVERFLOW: u8 = 0xE4;
/// The halt code for a boot with no memory map to hand over.
const NO_MAP: u8 = 0xE6;

/// Boot services are gone, so a map that does not fit cannot be retried:
/// the boot halts rather than run on a map cut short.
pub fn final_map(
    allocs: &HandoffAllocations,
    bh_ptr: *mut BootHandoffV1,
    final_mmap: &UefiMemoryMap<'_>,
) -> &'static [MemoryMapEntry] {
    let copied = copy_memory_map(allocs.mmap_addr, allocs.mmap_capacity, final_mmap);
    let Ok((mmap_ptr, entry_size, entry_count)) = copied else {
        crate::arch::cpu::halt_with_code(MAP_OVERFLOW)
    };
    finalize_mmap(bh_ptr, mmap_ptr, entry_size, entry_count);
    // No map means no list of what to clean for the kernel, and no free RAM
    // for it to take: the boot stops rather than guess.
    if mmap_ptr == 0 || entry_count == 0 {
        crate::arch::cpu::halt_with_code(NO_MAP)
    }
    // SAFETY: `copy_memory_map` wrote `entry_count` entries at `mmap_ptr`,
    // loader data that lives until the kernel takes over.
    unsafe { core::slice::from_raw_parts(mmap_ptr as *const MemoryMapEntry, entry_count as usize) }
}
