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

use crate::boot::handoff::BootHandoffV1;
use crate::memory::addr::PhysAddr;
use crate::sys::serial;

use super::low_dma::find_low_dma_region;
use super::memory_span::{reserve_gaps, say_managed};

pub(crate) fn init_memory(handoff: &BootHandoffV1) {
    /*
     * The allocator used to be given only the largest usable region, so a
     * machine with its memory in several regions ran on one of them: on q35
     * the 2 GiB below the PCI hole whatever the machine had, and a model
     * that needed more than what was left there could not load. It now
     * manages the span from the lowest usable frame to the highest, with
     * every frame between regions marked in use before anything allocates.
     */
    let (mut mem_start, mut mem_end) = (u64::MAX, 0u64);
    unsafe {
        for (start, end) in handoff.mmap.usable_regions() {
            let start = start.max(0x100000);
            if end > start {
                mem_start = mem_start.min(start);
                mem_end = mem_end.max(end);
            }
        }
    }
    if mem_end <= mem_start || mem_end - mem_start < 0x100000 {
        mem_start = 0x100000;
        mem_end = 0x8000_0000;
    }
    mem_end = mem_end.min(crate::memory::phys::MAX_PHYSICAL_MEMORY);
    let start = PhysAddr::new(mem_start);
    let end = PhysAddr::new(mem_end);
    match crate::memory::phys::init(start, end) {
        Ok(()) => {
            reserve_gaps(handoff, mem_start, mem_end);
            say_managed(mem_start, mem_end);
        }
        Err(_) => {
            serial::println(b"[MEM] phys init failed, using fallback");
            init_fallback();
        }
    }
    if !crate::memory::phys::is_initialized() {
        serial::println(b"[MEM] CRITICAL: phys not initialized");
        init_fallback();
    }
    /*
     * The low DMA pool is carved from a usable region below 4 GiB, and is
     * the pool's own: its frames are reserved so the allocator never hands
     * the same frame out twice.
     */
    let (dma_base, dma_pages) = find_low_dma_region(handoff);
    crate::memory::phys::reserve(dma_base, dma_base + dma_pages as u64 * 0x1000);
    if crate::memory::phys::is_initialized() {
        crate::hardware::broker::dma::init_display_pool();
        crate::hardware::broker::dma::init_low32_pool(dma_base, dma_pages);
    }
}

fn init_fallback() {
    let regions = [
        (0x100000u64, 0x8000_0000u64),
        (0x100000u64, 0x4000_0000u64),
        (0x200000u64, 0x1000_0000u64),
    ];
    for (start, end) in regions {
        if crate::memory::phys::init(PhysAddr::new(start), PhysAddr::new(end)).is_ok() {
            crate::sys::serial::println(b"[MEM] fallback OK");
            return;
        }
    }
}
