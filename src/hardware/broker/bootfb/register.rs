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

//! The boot framebuffer published as a firmware-bus display device, so a
//! display driver can claim it and map it instead of the kernel and the
//! compositor sharing it (L31).

use alloc::format;
use spin::Once;

use super::rule::window;
use crate::hardware::broker::claim::FIRMWARE_BOOT_FRAMEBUFFER;
use crate::hardware::broker::class::ids;
use crate::hardware::broker::device::{Bar, BarKind, BusKind, DeviceRecord, BAR_FLAG_PREFETCH};
use crate::hardware::broker::table::register_platform_device;
use crate::kernel_core::init::framebuffer::phys_window;
use crate::sys::serial;

static BOOTFB: Once<u64> = Once::new();

pub fn register_boot_framebuffer() {
    let Some(w) = phys_window() else {
        return;
    };
    // Whether published or not, a claim of the memory it lies in stops the
    // kernel drawing on it (L79).
    super::cover::find(w.page, w.len);
    let Some((base, size)) = window(w.page, w.len, w.offset) else {
        serial::println(b"[BROKER] boot framebuffer: its window is malformed, not published");
        return;
    };
    if !super::sink::make() {
        serial::println(b"[BROKER] boot framebuffer: no frame for its sink, not published");
        return;
    }
    let mut bars = [Bar::empty(); 6];
    // Prefetchable, so a driver may ask for write-combining; the kernel's own
    // mapping still decides the type it gets (L17).
    let bar = BarKind::Mmio as u8;
    bars[0] = Bar { base, size, aux: w.offset, kind: bar, flags: BAR_FLAG_PREFETCH, _pad: [0; 2] };
    let id = register_platform_device(DeviceRecord {
        bus_kind: BusKind::Firmware as u8,
        class: ids::DISPLAY,
        device: FIRMWARE_BOOT_FRAMEBUFFER,
        bar_count: 1,
        bars,
        ..DeviceRecord::empty()
    });
    BOOTFB.call_once(|| id);
    let line =
        format!("[BROKER] boot framebuffer {:#x}+{:#x} published as device {}", base, size, id);
    serial::println(line.as_bytes());
}

pub(in crate::hardware::broker) fn is_boot_framebuffer(device_id: u64) -> bool {
    BOOTFB.get() == Some(&device_id)
}

pub(in crate::hardware::broker) fn device() -> Option<u64> {
    BOOTFB.get().copied()
}
