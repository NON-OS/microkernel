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

//! The ACPI records a platform bus controller's key is found among: each
//! ACPI-bus record's namespace path and device id, for
//! `nonos_gpio::bus_key` to match SERIAL's bus-controller path against.

use crate::name::name;
use alloc::vec::Vec;
use nonos_libc::{mk_device_list, DeviceRecord, BUS_KIND_ACPI};

const MAX_DEVICES: usize = 128;

pub fn platform_paths() -> Vec<(Vec<u8>, u64)> {
    let mut buf = [DeviceRecord::empty(); MAX_DEVICES];
    let n = mk_device_list(0, buf.as_mut_ptr(), MAX_DEVICES as u64);
    let count = if n > 0 { (n as usize).min(MAX_DEVICES) } else { 0 };
    let acpi = buf[..count].iter().filter(|r| r.bus_kind == BUS_KIND_ACPI);
    acpi.filter_map(|r| Some((name(r.device_id)?.path, r.device_id)))
        .filter(|(p, _)| !p.is_empty())
        .collect()
}
