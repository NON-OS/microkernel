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

//! The kernel image and the boot devices in the boot tables. RAM is mapped
//! first as 2 MiB of data, never executable; the image is then mapped again
//! one 4 KiB page at a time by section: text read and execute, read-only data
//! read only, the rest read and write. Devices are mapped read and write and
//! never executable.

use super::map::map_page;
use super::PageAttributes;
use crate::arch::riscv64::boot::info::BootInfo;

const PAGE: u64 = 0x1000;
/// The PLIC's whole window: the specification gives it 64 MiB.
const PLIC_WINDOW: u64 = 0x400_0000;

extern "C" {
    static __kernel_text_start: u8;
    static __kernel_text_end: u8;
    static __kernel_rodata_start: u8;
    static __kernel_rodata_end: u8;
    static __kernel_image_end: u8;
}

fn addr(sym: &u8) -> u64 {
    sym as *const u8 as u64
}

fn pages(start: u64, end: u64, attrs: PageAttributes) {
    let mut at = start & !(PAGE - 1);
    while at < end {
        map_page(at, at, attrs);
        at += PAGE;
    }
}

/// # Safety
/// Boot hart, before `satp` points at these tables.
pub(super) unsafe fn map_image_and_devices(info: &BootInfo) {
    // SAFETY: linker symbols; only their addresses are taken.
    let (text, text_end, ro, ro_end, end) = unsafe {
        (
            addr(&__kernel_text_start),
            addr(&__kernel_text_end),
            addr(&__kernel_rodata_start),
            addr(&__kernel_rodata_end),
            addr(&__kernel_image_end),
        )
    };
    pages(text, text_end, PageAttributes::kernel_code());
    pages(ro, ro_end, PageAttributes::kernel_rodata());
    pages(ro_end, end, PageAttributes::kernel_data());
    if info.uart_base != 0 {
        pages(info.uart_base, info.uart_base + PAGE, PageAttributes::kernel_data());
    }
    if info.plic_base != 0 {
        pages(info.plic_base, info.plic_base + PLIC_WINDOW, PageAttributes::kernel_data());
    }
}
