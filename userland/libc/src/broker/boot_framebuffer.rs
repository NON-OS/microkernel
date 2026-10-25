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

//! The boot framebuffer as a device (L31). Its record is on the firmware bus
//! (`BUS_KIND_FIRMWARE`, device `FIRMWARE_BOOT_FRAMEBUFFER`, class display),
//! with bar 0 the window from the first pixel's page to the end of the last
//! row, prefetchable, its `aux` the first pixel's offset into that page. Only
//! the process serving `driver.bootfb0` may claim it. `mk_boot_framebuffer`
//! answers the rest, for the holder only. Cap requirement: `Mmio`.
//!
//! Once a GPU driver's claim takes the scanout (L79), the holder's mapping
//! shows a kernel sink page, `mk_boot_framebuffer` answers -ENODEV for good,
//! a release -ENODEV and a new claim -EPERM for the rest of the boot. No DMA
//! is ever granted against this record.

use core::mem::size_of;

use crate::syscall::{call_raw, N_MK_BOOT_FRAMEBUFFER};

/// The firmware bus's device number for the boot framebuffer.
pub const FIRMWARE_BOOT_FRAMEBUFFER: u16 = 0x0001;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BootFramebufferInfo {
    pub width: u32,
    pub height: u32,
    /// Bytes from one row to the next.
    pub stride: u32,
    /// The loader's code: 0 RGB, 1 BGR, 2 RGBX, 3 BGRX.
    pub format: u32,
    /// The first pixel's offset into bar 0.
    pub offset: u32,
    /// Width in millimetres in the low 16 bits, height in the high 16; 0 when
    /// the panel did not say.
    pub phys_mm: u32,
}

/// 24 on success; -EPERM unless the caller holds the claim, -ENODEV for any
/// other device, -EINVAL for a short buffer.
#[no_mangle]
pub extern "C" fn mk_boot_framebuffer(device_id: u64, out: *mut BootFramebufferInfo) -> i64 {
    let len = size_of::<BootFramebufferInfo>() as u64;
    call_raw(N_MK_BOOT_FRAMEBUFFER, [device_id, out as u64, len, 0, 0, 0])
}
