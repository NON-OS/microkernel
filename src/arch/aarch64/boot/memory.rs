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

//! Handing the device tree's memory map to the physical allocator.
//!
//! The x86_64 path walks EFI descriptors; here the regions came out of the FDT
//! and are already normalised into [`MemoryRegion`]. What both paths owe the
//! allocator is the same: one contiguous managed range, plus a below-4 GiB pool
//! for devices whose DMA descriptors are only 32 bits wide.

mod dma_window;
mod regions;
mod reserve;

use super::info::BootInfo;
use crate::hardware::broker::dma;
use crate::memory::addr::PhysAddr;
use crate::sys::serial;
use dma_window::find_low_dma_region;
use regions::largest_available;

const PAGE_SIZE: u64 = 0x1000;

extern "C" {
    static __kernel_image_end: u8;
}

pub(crate) fn init_boot_memory(info: &BootInfo) {
    let Some((region_start, end)) = largest_available(info) else {
        serial::println(b"[MEM] no usable region in device tree");
        return;
    };

    // The image sits inside the region firmware calls available, so the
    // allocator has to be told where it ends. A frame holding kernel text is
    // not a fault the kernel survives to report: it is handed out as a fresh
    // page table, zeroed, and the next call into whatever lived there executes
    // zeros.
    let image_end = (&raw const __kernel_image_end) as u64;
    let start = if image_end > region_start && image_end < end {
        (image_end + PAGE_SIZE - 1) & !(PAGE_SIZE - 1)
    } else {
        region_start
    };
    if start >= end {
        serial::println(b"[MEM] nothing usable above the kernel image");
        return;
    }

    let (dma_base, dma_pages) = find_low_dma_region(info, start, end);

    if crate::memory::phys::init(PhysAddr::new(start), PhysAddr::new(end)).is_err() {
        serial::println(b"[MEM] phys init failed");
        return;
    }
    reserve::apply(info);
    serial::println(b"[MEM] phys init OK");

    dma::init_display_pool();
    dma::init_low32_pool(dma_base, dma_pages);
}
