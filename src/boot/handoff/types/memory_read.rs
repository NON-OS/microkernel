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

//! Reading the loader's memory map, through the direct map.

use super::map_window::map_window;
use super::memory::{memory_type, MemoryMap, MemoryMapEntry};
use crate::memory::layout::{DIRECTMAP_BASE, DIRECTMAP_SIZE};

impl MemoryMap {
    /// The entries, read through the direct map (`map_window`); none when
    /// the map is empty or the direct map does not hold it whole.
    // SAFETY: Caller must ensure ptr points to a MemoryMapEntry array the
    // loader left, `entry_count` long.
    pub unsafe fn entries(&self) -> &[MemoryMapEntry] {
        let size = core::mem::size_of::<MemoryMapEntry>() as u32;
        if self.entry_size != size {
            return &[];
        }
        match map_window(self.ptr, size, self.entry_count, DIRECTMAP_BASE, DIRECTMAP_SIZE) {
            // SAFETY: the window lies inside the direct map, which maps
            // every byte of it, and the caller vouches for its contents.
            Some((virt, n)) => unsafe {
                core::slice::from_raw_parts(virt as *const MemoryMapEntry, n)
            },
            None => &[],
        }
    }

    /// Usable regions as `[start, end)`; a region whose end does not fit a
    /// u64 ends at the top.
    // SAFETY: as `entries`.
    pub unsafe fn usable_regions(&self) -> impl Iterator<Item = (u64, u64)> + '_ {
        unsafe {
            self.entries()
                .iter()
                .filter(|e| e.memory_type == memory_type::CONVENTIONAL)
                .map(|e| (e.physical_start, region_end(e)))
        }
    }

    // SAFETY: as `entries`.
    pub unsafe fn total_usable_memory(&self) -> u64 {
        unsafe {
            self.entries()
                .iter()
                .filter(|e| e.memory_type == memory_type::CONVENTIONAL)
                .fold(0u64, |t, e| t.saturating_add(e.page_count.saturating_mul(4096)))
        }
    }
}

fn region_end(e: &MemoryMapEntry) -> u64 {
    e.physical_start.saturating_add(e.page_count.saturating_mul(4096))
}
