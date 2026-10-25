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

//! The memory map handed to the kernel, copied from the firmware's after
//! ExitBootServices; pure, so it is proven on the host. The buffer held 1024
//! entries and the copy stopped there without a word, so a machine with
//! more descriptors booted on a map missing whatever came after. The buffer
//! is now sized from the firmware's own map, and a map that still does not
//! fit is refused rather than cut.

use super::types::MemoryMapEntry;

/// Descriptors the map may gain between its size being read and
/// ExitBootServices: each allocation after the read, the kernel's page
/// tables among them, can split a region in two.
pub const MMAP_SLACK: usize = 256;
/// The fewest entries the buffer holds, whatever the firmware says.
pub const MMAP_FLOOR: usize = 1024;

/// A map with more descriptors than the buffer holds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MapOverflow {
    pub entries: usize,
    pub room: usize,
}

/// Entries to make room for, from the firmware's map size and descriptor
/// size as `GetMemoryMap` reports them.
pub const fn mmap_capacity(map_size: usize, desc_size: usize) -> usize {
    let entries = if desc_size == 0 { 0 } else { map_size.div_ceil(desc_size) };
    let wanted = entries.saturating_add(MMAP_SLACK);
    if wanted < MMAP_FLOOR {
        MMAP_FLOOR
    } else {
        wanted
    }
}

/// Pages holding `capacity` entries.
pub const fn mmap_pages(capacity: usize) -> usize {
    capacity.saturating_mul(MemoryMapEntry::SIZE).div_ceil(0x1000)
}

/// Copy every entry of `src` into `out`. All or nothing in what it returns:
/// a map with more entries than `out` holds is `MapOverflow`, its whole
/// count named, and must not be handed on.
pub fn copy_entries(
    src: impl Iterator<Item = MemoryMapEntry>,
    out: &mut [MemoryMapEntry],
) -> Result<u32, MapOverflow> {
    let mut entries = 0usize;
    for entry in src {
        if let Some(slot) = out.get_mut(entries) {
            *slot = entry;
        }
        entries = entries.saturating_add(1);
    }
    if entries > out.len() {
        return Err(MapOverflow { entries, room: out.len() });
    }
    u32::try_from(entries).map_err(|_| MapOverflow { entries, room: out.len() })
}
