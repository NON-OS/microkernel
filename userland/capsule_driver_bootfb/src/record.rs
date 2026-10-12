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

//! The boot framebuffer as the kernel lists it (lead L31): a firmware
//! record, class display, device FIRMWARE_BOOT_FRAMEBUFFER, vendor 0, one
//! bar: MMIO, prefetchable, from the page the first pixel lies in to the
//! end of the last row, its aux the first pixel's offset into that page.
//! Read here from the fields the binary copies out of libc's DeviceRecord,
//! so the match is proved without the broker.

/// `MkDeviceList`'s firmware bus (4 is C's device tree), the display
/// class, and the boot framebuffer's device number.
pub const BUS_KIND_FIRMWARE: u8 = 5;
pub const CLASS_DISPLAY: u32 = 0x0030;
pub const FIRMWARE_BOOT_FRAMEBUFFER: u16 = 0x0001;
/// libc's BAR_KIND_MMIO and BAR_FLAG_PREFETCH.
pub const BAR_KIND_MMIO: u8 = 1;
pub const BAR_FLAG_PREFETCH: u8 = 1 << 0;

/// One listed device, as far as bootfb looks at it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Listed {
    pub device_id: u64,
    pub bus_kind: u8,
    pub class: u32,
    pub vendor: u16,
    pub device: u16,
    pub bar_count: u8,
    /// Bar 0: kind, flags, base, size, aux.
    pub bar0: (u8, u8, u64, u64, u32),
}

/// The boot framebuffer's record, and the bar size to map; None for any
/// other device or a bar that is not the one the kernel describes.
pub fn boot_fb(r: &Listed) -> Option<u64> {
    let (kind, flags, base, size, aux) = r.bar0;
    let ours = r.bus_kind == BUS_KIND_FIRMWARE
        && r.class == CLASS_DISPLAY
        && r.device == FIRMWARE_BOOT_FRAMEBUFFER
        && r.vendor == 0
        && r.bar_count >= 1;
    let bar = kind == BAR_KIND_MMIO
        && flags & BAR_FLAG_PREFETCH != 0
        && base.is_multiple_of(4096)
        && (aux as u64) < size.min(4096);
    (ours && bar).then_some(size)
}
