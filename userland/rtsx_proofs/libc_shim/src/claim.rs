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

//! The claim and the release. A release does what the kernel's
//! MkDeviceRelease does: bus mastering off, read back, then every grant of
//! the device drained. The device is given a turn right after the release
//! clears bus mastering, the moment a late completion would race the
//! teardown. The PCI command write is in command.rs.

use std::cell::Cell;

use crate::events::note;
use crate::hook::device_acts;

/// The claim epoch every claim gets here.
pub const EPOCH: i64 = 7;

thread_local! {
    static BUS_MASTER: Cell<bool> = const { Cell::new(false) };
}

pub fn mk_device_claim(device_id: u64) -> i64 {
    note(format!("claim {device_id:#x}"));
    EPOCH
}

pub fn mk_device_release(device_id: u64) -> i64 {
    note(format!("release {device_id:#x}"));
    set_bus_master(false);
    device_acts();
    crate::grants::drain(device_id);
    0
}

/// Whether the device may master the bus now: a DMA it starts lands only then.
pub fn bus_master() -> bool {
    BUS_MASTER.with(|b| b.get())
}

pub(crate) fn set_bus_master(on: bool) {
    BUS_MASTER.with(|b| b.set(on));
}
