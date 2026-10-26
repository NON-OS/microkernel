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

//! Handing free RAM (`ram`: a NONOS loader's EFI map, else the device
//! tree's) to the physical allocator: one contiguous managed range, a
//! below-4 GiB pool for 32-bit DMA, and the uncached block cut from that pool.

mod clip;
mod dma_place;
mod dma_size;
mod dma_window;
mod efi_runs;
mod image;
mod loader_map;
mod ram;
mod regions;
mod reserve;
mod span;
mod uncached;
mod usable;

use super::info::BootInfo;
use crate::hardware::broker::dma;
use crate::memory::addr::PhysAddr;
use crate::sys::serial;
use dma_window::find_low_dma_region;

const PAGE_SIZE: u64 = 0x1000;

pub(crate) fn init_boot_memory(info: &BootInfo) {
    let ram = ram::ram(info).unwrap_or_else(|reason| super::refuse(b"boot memory", reason));
    // The runs are held to what the direct map reaches (`ram`).
    let Some((region_start, end)) = ram.usable.largest() else {
        serial::println(b"[MEM] no usable RAM region");
        return;
    };
    // The image sits inside the region firmware calls available, so the
    // allocator has to be told where it ends. A frame holding kernel text is
    // handed out as a fresh page table, zeroed, and the next call into
    // whatever lived there executes zeros.
    let image_bounds @ (image_start, image_end) = image::bounds();
    let start = span::span_start((region_start, end), (image_start, image_end));
    if start >= end {
        serial::println(b"[MEM] nothing usable above the kernel image");
        return;
    }

    // The image is in no list the device tree gave, so it is named here: a
    // pool placed over it hands kernel text to a capsule as a DMA buffer.
    let (runs, reserved) = (ram.usable.runs(), info.reserved.ranges());
    let pool = find_low_dma_region(runs, reserved, (start, end), image_bounds);
    if crate::memory::phys::init(PhysAddr::new(start), PhysAddr::new(end)).is_err() {
        serial::println(b"[MEM] phys init failed");
        return;
    }
    reserve::apply(info, &ram, pool.as_ref());
    serial::println(b"[MEM] phys init OK");
    let pool = pool.map(uncached::cut);
    dma::init_display_pool();
    let (dma_base, dma_pages) = pool.map_or((0, 0), |p| (p.base, p.pages));
    dma::init_low32_pool(dma_base, dma_pages);
}
