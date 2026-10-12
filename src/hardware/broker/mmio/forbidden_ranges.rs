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

//! The physical ranges forbidden.rs refuses, gathered once: RAM from the
//! boot memory map, the kernel image as it is mapped, each IOMMU's
//! registers. None when the memory map is missing or malformed, and every
//! MMIO grant is then refused: without it no BAR can be shown not to be RAM.

extern crate alloc;

use alloc::vec::Vec;
use core::mem::size_of;

use spin::Once;

use super::forbidden::{is_ram, page_span, touches};
use super::types::MmioMapError;
use crate::boot::handoff::{get_handoff, MemoryMapEntry};
use crate::memory::addr::PhysAddr;

/// The bootloader's own bound (prepare/constants.rs MAX_MMAP_ENTRIES).
const MAX_ENTRIES: usize = 1024;

static RANGES: Once<Option<Vec<(u64, u64)>>> = Once::new();

/// Ok when the pages `[start, start + bytes)` touch no forbidden range.
pub(super) fn admit(start: u64, bytes: u64) -> Result<(), MmioMapError> {
    let end = start.checked_add(bytes).ok_or(MmioMapError::Overflow)?;
    let ranges = RANGES.call_once(gather).as_deref().ok_or(MmioMapError::Forbidden)?;
    match touches(start, end, ranges) {
        true => Err(MmioMapError::Forbidden),
        false => Ok(()),
    }
}

fn gather() -> Option<Vec<(u64, u64)>> {
    let mut out = ram()?;
    super::forbidden_image::push(&mut out);
    super::forbidden_iommu::push(&mut out);
    Some(out)
}

fn ram() -> Option<Vec<(u64, u64)>> {
    let map = get_handoff()?.mmap;
    let count = map.entry_count as usize;
    if map.ptr == 0 || count == 0 || count > MAX_ENTRIES {
        return None;
    }
    if map.entry_size as usize != size_of::<MemoryMapEntry>() {
        return None;
    }
    let base = crate::memory::unified::phys_to_virt(PhysAddr::new(map.ptr))?.as_u64();
    // SAFETY: eK@nonos.systems - init_handoff checked ptr below the 48-bit
    // ceiling, and entry_size is MemoryMapEntry's; the bootloader put the
    // map in LOADER_DATA, which the kernel never takes back, and the
    // directmap covers it; count is within the bootloader's bound.
    let entries = unsafe { core::slice::from_raw_parts(base as *const MemoryMapEntry, count) };
    let ram = entries.iter().filter(|e| is_ram(e.memory_type));
    ram.map(|e| page_span(e.physical_start, e.page_count)).collect()
}
