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

//! The frames between the firmware's usable regions, and a line saying
//! what the allocator was given.

use crate::boot::handoff::BootHandoffV1;
use crate::sys::serial;

/*
 * Mark in use every frame of the managed span that no usable region covers:
 * holes, MMIO windows, firmware and the loader's own data. What the regions
 * cover is the RAM every total-memory figure reports, not the span.
 */
pub(super) fn reserve_gaps(handoff: &BootHandoffV1) {
    let mut usable: alloc::vec::Vec<(u64, u64)> =
        unsafe { handoff.mmap.usable_regions().collect() };
    usable.sort_unstable();
    crate::memory::phys::keep_usable(&usable);
    let mut ram: alloc::vec::Vec<(u64, u64)> = unsafe { handoff.mmap.ram_regions().collect() };
    ram.sort_unstable();
    crate::memory::phys::keep_ram(&ram);
}

/// `[MEM] 3967 MiB free to allocate, in a span of 0x100000..0x180000000`.
pub(super) fn say_managed(span_start: u64, span_end: u64) {
    let free = crate::memory::phys::total_free_frames() as u64 * 4096;
    let mut l = serial::Line::new();
    l.str(b"[MEM] ").dec(free >> 20).str(b" MiB free to allocate, in a span of ");
    l.hex(span_start).str(b"..").hex(span_end).end();
}
