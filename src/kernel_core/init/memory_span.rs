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

/// Mark in use every frame of `[span_start, span_end)` that no usable
/// region covers: holes, MMIO windows, firmware and the loader's own data.
pub(super) fn reserve_gaps(handoff: &BootHandoffV1, span_start: u64, span_end: u64) {
    let mut usable: alloc::vec::Vec<(u64, u64)> =
        unsafe { handoff.mmap.usable_regions().collect() };
    usable.sort_unstable();
    let mut at = span_start;
    for (start, end) in usable {
        if start > at {
            crate::memory::phys::reserve(at, start.min(span_end));
        }
        at = at.max(end);
    }
    if at < span_end {
        crate::memory::phys::reserve(at, span_end);
    }
}

/// `[MEM] 3967 MiB free to allocate, in a span of 0x100000..0x180000000`.
pub(super) fn say_managed(span_start: u64, span_end: u64) {
    let free = crate::memory::phys::total_free_frames() as u64 * 4096;
    serial::print(b"[MEM] ");
    serial::print_dec(free >> 20);
    serial::print(b" MiB free to allocate, in a span of ");
    serial::print_hex(span_start);
    serial::print(b"..");
    serial::print_hex(span_end);
    serial::println(b"");
}
