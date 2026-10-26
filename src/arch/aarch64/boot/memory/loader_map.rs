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

//! The EFI memory map a NONOS loader left, read through the direct map once
//! every check that makes the read safe has passed.

use crate::arch::aarch64::boot::info::BootInfo;
use crate::arch::aarch64::boot::loader::placed;
use crate::boot::handoff::{BootHandoffV1, MemoryMapEntry};

/// The map's entries; an error when the handoff names no map this kernel
/// reads, or one outside RAM or the direct map.
pub(super) fn entries(
    info: &BootInfo,
    handoff: &'static BootHandoffV1,
) -> Result<&'static [MemoryMapEntry], &'static [u8]> {
    let map = &handoff.mmap;
    let size = core::mem::size_of::<MemoryMapEntry>() as u64;
    if map.entry_count == 0 || u64::from(map.entry_size) != size {
        return Err(b"the loader's handoff has no memory map this kernel reads");
    }
    let len = size * u64::from(map.entry_count);
    let in_ram = placed(info.memory_map().iter().map(|r| (r.base, r.size)), map.ptr, len);
    let mapped = map
        .ptr
        .checked_add(len)
        .is_some_and(|end| end <= crate::arch::aarch64::mmu::direct_map_end());
    if !in_ram || !mapped {
        return Err(b"the loader's memory map lies outside RAM or the direct map");
    }
    // SAFETY: the loader left `entry_count` entries at `ptr` (the handoff's
    // magic, version and size were checked when it was taken), in RAM the
    // tree names and the direct map reaches, checked above.
    let entries = unsafe { handoff.mmap.entries() };
    if entries.len() != map.entry_count as usize {
        return Err(b"the loader's memory map lies outside the direct map");
    }
    Ok(entries)
}
