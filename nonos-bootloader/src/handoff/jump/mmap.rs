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

use super::mmap_copy::{copy_entries, MapOverflow};
use super::types::MemoryMapEntry;
use crate::handoff::types::{BootHandoffV1, MemoryMap};
use uefi::table::boot::MemoryMap as UefiMemoryMap;

/// Copy the UEFI memory map into the kernel's format, after
/// ExitBootServices: the address, the entry size and the count, or the
/// overflow when the map does not fit the `capacity` entries allocated.
pub fn copy_memory_map(
    mmap_addr: u64,
    capacity: usize,
    final_mmap: &UefiMemoryMap<'_>,
) -> Result<(u64, u32, u32), MapOverflow> {
    if mmap_addr == 0 {
        return Ok((0, 0, 0));
    }
    // SAFETY: `allocate_handoff_resources` allocated `capacity` entries at
    // `mmap_addr`, loader data that nothing else holds.
    let out =
        unsafe { core::slice::from_raw_parts_mut(mmap_addr as *mut MemoryMapEntry, capacity) };
    let entries = final_mmap.entries().map(|desc| MemoryMapEntry {
        memory_type: desc.ty.0,
        _pad: 0,
        physical_start: desc.phys_start,
        virtual_start: desc.virt_start,
        page_count: desc.page_count,
        attribute: desc.att.bits(),
    });
    let count = copy_entries(entries, out)?;
    Ok((mmap_addr, MemoryMapEntry::SIZE as u32, count))
}

/// Write final memory map info to handoff struct.
pub fn finalize_mmap(
    bh_ptr: *mut BootHandoffV1,
    mmap_addr: u64,
    entry_size: u32,
    entry_count: u32,
) {
    if bh_ptr.is_null() {
        return;
    }
    // SAFETY: bh_ptr validated by caller, points to allocated BootHandoffV1
    unsafe {
        (*bh_ptr).mmap = MemoryMap { ptr: mmap_addr, entry_size, entry_count, desc_version: 1 };
    }
}
