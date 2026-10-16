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
//! each behind a mapping whose length the shim's broker reports. The memory
//! behind the framebuffer is longer than that length, so a write past the
//! mapping lands where a test reads it instead of in the test's own heap.

use std::sync::Arc;

use nonos_devmodel::FakeBar;
use nonos_libc::map_bar;

use crate::constants::{BYTES_PER_PIXEL, DISPI_ID_LAST, DISPI_INDEX_ENABLE, DISPI_INDEX_ID};
use crate::constants::{FB_BAR, MODE_HEIGHT, MODE_WIDTH, REG_BAR};
use crate::discover::Found;
use crate::dispi::dispi_off;

pub const MODE_BYTES: usize = MODE_WIDTH as usize * MODE_HEIGHT as usize * BYTES_PER_PIXEL as usize;
pub const GUARD: u32 = 0xdead_beef;
pub const REG_LEN: usize = 0x1000;
pub const DEVICE: u64 = 9;

pub struct Adapter {
    pub regs: Arc<FakeBar>,
    pub fb: FakeBar,
}

/// A QEMU adapter (DISPI ID 0xB0C5) whose register and framebuffer mappings
/// the broker reports as `regs` and `fb` bytes long, with a guard word at
/// the end of each.
pub fn adapter(regs: usize, fb: usize) -> Adapter {
    let window = Arc::new(FakeBar::new(REG_LEN + 4));
    let a = Adapter { regs: window, fb: FakeBar::new(MODE_BYTES.max(fb) + 4) };
    a.regs.present16(dispi_off(DISPI_INDEX_ID) as usize, DISPI_ID_LAST);
    a.regs.present32(regs.min(REG_LEN), GUARD);
    a.fb.present32(fb, GUARD);
    map_bar(REG_BAR, a.regs.base(), regs as u64);
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
