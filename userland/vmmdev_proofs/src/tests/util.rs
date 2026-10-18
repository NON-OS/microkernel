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

//! A guest device and a driver up on it.

use nonos_libc::{set_devices, Bar, DeviceRecord, BAR_KIND_MMIO, BAR_KIND_PIO, BUS_KIND_PCI};

use crate::discover::Found;
use crate::model::{install, Options, Shared};
use crate::setup::{run, Driver};

pub const DEVICE: u64 = 4;
pub const BAR1: u64 = 0x0040_0000;

/// The record the broker lists for VirtualBox's guest device.
pub fn record() -> DeviceRecord {
    let mut r = DeviceRecord {
        device_id: DEVICE,
        bus_kind: BUS_KIND_PCI,
        vendor: 0x80EE,
        device: 0xCAFE,
        bar_count: 2,
        ..Default::default()
    };
    r.bars[0] = Bar { kind: BAR_KIND_PIO, size: 32 };
    r.bars[1] = Bar { kind: BAR_KIND_MMIO, size: BAR1 };
    r
}

pub fn up_with(opt: Options) -> (Result<Driver, &'static str>, Shared) {
    let host = install(opt);
    set_devices(vec![record()]);
    (run(Found { device_id: DEVICE, memory_bytes: BAR1 }), host)
}

pub fn up() -> (Driver, Shared) {
    let (d, host) = up_with(Options::default());
    (d.expect("bring-up"), host)
}
