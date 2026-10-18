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

//! An adapter for one bring-up attempt: a register window and a framebuffer,
//! each behind a mapping whose length the shim's broker reports, and the
//! memory the shim hands out for the driver's surface. The memory behind the
//! framebuffer is longer than that length, so a write past the mapping lands
//! where a test reads it instead of in the test's own heap. The shim records
//! the DISPI block at each give-back and poisons the register mapping from
//! its unmap to the release.

use std::sync::Arc;

use nonos_devmodel::FakeBar;
use nonos_libc::{lend, map_bar, watch};

use crate::constants::{DISPI_ID_LAST, DISPI_INDEX_ENABLE, DISPI_INDEX_ID};
use crate::constants::{DISPI_INDEX_VIDEO_MEMORY_64K, FALLBACK_MODE, FB_BAR, REG_BAR};
use crate::discover::Found;
use crate::dispi::{dispi_off, regs_len};

/// The mode set when firmware left none and the monitor names none.
pub const MODE: (u32, u32) = FALLBACK_MODE;
pub const MODE_BYTES: usize = MODE.0 as usize * MODE.1 as usize * 4;
pub const GUARD: u32 = 0xdead_beef;
pub const REG_LEN: usize = 0x1000;
pub const DEVICE: u64 = 9;
/// QEMU's default: 16 MiB of video memory, in 64 KiB units.
pub const VIDEO_64K: u16 = 256;

pub struct Adapter {
    pub regs: Arc<FakeBar>,
    pub fb: FakeBar,
    pub surface: FakeBar,
}

/// A QEMU adapter (DISPI ID 0xB0C5, 16 MiB of video memory) whose register
/// and framebuffer mappings the broker reports as `regs` and `fb` bytes
/// long, with a guard word at the end of each.
pub fn adapter(regs: usize, fb: usize) -> Adapter {
    let window = Arc::new(FakeBar::new(REG_LEN + 4));
    let room = MODE_BYTES.max(fb);
    let a = Adapter { regs: window, fb: FakeBar::new(room + 4), surface: FakeBar::new(room) };
    a.regs.present16(dispi_off(DISPI_INDEX_ID) as usize, DISPI_ID_LAST);
    a.regs.present16(dispi_off(DISPI_INDEX_VIDEO_MEMORY_64K) as usize, VIDEO_64K);
    a.regs.present32(regs.min(REG_LEN), GUARD);
    lend(a.surface.base(), room);
    a.fb.present32(fb, GUARD);
    map_bar(REG_BAR, a.regs.base(), regs as u64);
    watch(REG_BAR, dispi_off(DISPI_INDEX_ID) as usize..regs_len() as usize);
    map_bar(FB_BAR, a.fb.base(), fb as u64);
    a
}

pub fn found(fb_size: usize) -> Found {
    Found { device_id: DEVICE, fb_size: fb_size as u64, reg_size: REG_LEN as u64 }
}

/// DISPI ENABLE as the attempt left it.
pub fn enabled(a: &Adapter) -> u16 {
    a.regs.wrote16(dispi_off(DISPI_INDEX_ENABLE) as usize)
}
