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

//! Handing the device tree's RAM to the physical allocator: the widest
//! Available region, starting past the kernel image when the two meet, with
//! every reserved range (the blob, the SBI firmware's own) taken out.

use super::info::{BootInfo, MemoryType};
use super::memory_span::{inward, span_start};
use crate::hardware::broker::dma;
use crate::memory::addr::PhysAddr;
use crate::sys::serial;

extern "C" {
    static __kernel_image_start: u8;
    static __kernel_image_end: u8;
}

pub(crate) fn init_boot_memory(info: &BootInfo) {
    let free = info.memory_map().iter().filter(|r| r.region_type == MemoryType::Available);
    let Some((start, end)) = free.filter_map(|r| inward(r.base, r.size)).max_by_key(|r| r.1 - r.0)
    else {
        stop(b"[MEM] riscv64: the device tree names no usable RAM; stopping")
    };
    // The kernel runs where it is linked, so its symbols are physical.
    // SAFETY: linker symbols; only their addresses are taken.
    let image = unsafe {
        (&__kernel_image_start as *const u8 as u64, &__kernel_image_end as *const u8 as u64)
    };
    let start = span_start((start, end), image);
    if start >= end {
        stop(b"[MEM] riscv64: nothing usable above the kernel image; stopping")
    }
    if crate::memory::phys::init(PhysAddr::new(start), PhysAddr::new(end)).is_err() {
        stop(b"[MEM] riscv64: phys init failed; stopping")
    }
    for &(a, b) in info.reserved.ranges() {
        crate::memory::phys::reserve(a, b);
    }
    serial::println(b"[MEM] phys init OK");
    // No low 32-bit pool: no riscv64 device is given DMA yet (no IOMMU).
    dma::init_display_pool();
    dma::init_low32_pool(0, 0);
}

fn stop(why: &[u8]) -> ! {
    serial::println(why);
    crate::arch::halt_loop()
}
