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

//! The windows the driver is pointed at. One window carries the common
//! configuration at zero, the notify region at `NOTIFY_AT` and the device
//! configuration at `DEVICE_AT`, as three capabilities would place them.
//! Feature pages read from one register after a select write, and a passive
//! window cannot tell the pages apart, so both offer what is presented
//! there: bit 0 is VIRGL low and VERSION_1 high, bit 1 EDID and
//! ACCESS_PLATFORM.

use std::sync::Arc;

use nonos_devmodel::FakeBar;
use nonos_virtio::common::{self as cfg, NotifyArea};
use nonos_virtio::Mmio;

use crate::init::{negotiate, program};
use crate::regs::Regs;

pub const NOTIFY_AT: usize = 0x100;
pub const NOTIFY_LEN: usize = 0x100;
pub const NOTIFY_MULT: u32 = 4;
pub const NOTIFY_OFF: u16 = 3;
pub const DEVICE_AT: usize = 0x200;
pub const QUEUE_SIZE: u16 = 64;
/// Where a bring-up says the ring region is; nothing reads it.
pub const REGION_PHYS: u64 = 0x1_2345_0000;

pub fn modern_window(offered: u32, queue_size: u16) -> Arc<FakeBar> {
    let bar = Arc::new(FakeBar::new(0x300));
    bar.present32(cfg::DEVICE_FEATURE, offered);
    bar.present16(cfg::QUEUE_SIZE, queue_size);
    bar.present16(cfg::QUEUE_NOTIFY_OFF, NOTIFY_OFF);
    bar
}

/// `len` bytes of `bar` from `at`, as the view the driver is handed.
pub fn view(bar: &FakeBar, at: usize, len: usize) -> Mmio {
    assert!(at + len <= bar.len(), "a view past the window");
    // SAFETY: inside the window (checked above), whose cells live as long
    // as `bar`; every test keeps `bar` until it is done with the view.
    unsafe { Mmio::new((bar.base() as usize + at) as *mut u8, len) }
}

/// The three regions, before the control queue's doorbell is known.
pub fn regs(bar: &FakeBar) -> Regs {
    let (common, device) = (view(bar, 0, NOTIFY_AT), view(bar, DEVICE_AT, 0x100));
    Regs { common, notify: view(bar, NOTIFY_AT, NOTIFY_LEN), doorbell: 0, device }
}

/// The bring-up as `setup::transport` runs it, less the grant and bus
/// mastering: the features in force, the queue size, the doorbell.
pub fn bring_up(bar: &FakeBar, notify_len: usize) -> Result<(u64, u16, usize), &'static str> {
    let common = regs(bar).common;
    let features = negotiate(&common)?;
    let area = NotifyArea { multiplier: NOTIFY_MULT, len: notify_len };
    let started = program(&common, area, REGION_PHYS)?;
    cfg::driver_ok(&common);
    Ok((features, started.queue_size, started.doorbell))
}
