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

//! The key a device's pin is bound to: the device's bus controller, whose
//! holder alone hears the pin. A controller that is a PCI function (Intel
//! LPSS) is named by it, segment << 16 | bus << 8 | devfn from its `_ADR`
//! (segment 0, bus 0: the root bus these controllers sit on). One that is
//! an ACPI platform device (AMD's AMDI0010) has no PCI function: it is
//! named by its broker device id with `PLATFORM` set, found by matching
//! SERIAL's bus-controller path against each ACPI record's name. Either
//! key is what capsule_acpi's HOLDER takes.

use crate::name::name;
use alloc::vec::Vec;
use nonos_libc::{mk_device_list, DeviceRecord, BUS_KIND_ACPI};

pub const PLATFORM: u64 = 1 << 63;
const MAX_DEVICES: usize = 128;
/// Where SERIAL's strings start: device path, `_HID`, bus controller path,
/// GPIO controller path, a length byte and bytes each.
const STRINGS: usize = 42;
const ADR: usize = 18;

/// Every ACPI-bus record's namespace path and device id.
pub fn platform_paths() -> Vec<(Vec<u8>, u64)> {
    let mut buf = [DeviceRecord::empty(); MAX_DEVICES];
    let n = mk_device_list(0, buf.as_mut_ptr(), MAX_DEVICES as u64);
    let count = if n > 0 { (n as usize).min(MAX_DEVICES) } else { 0 };
    let acpi = buf[..count].iter().filter(|r| r.bus_kind == BUS_KIND_ACPI);
    acpi.filter_map(|r| Some((name(r.device_id)?.path, r.device_id)))
        .filter(|(p, _)| !p.is_empty())
        .collect()
}

fn string(r: &[u8], nth: usize) -> Option<&[u8]> {
    let mut at = STRINGS;
    for _ in 0..nth {
        at += 1 + usize::from(*r.get(at)?);
    }
    let len = usize::from(*r.get(at)?);
    r.get(at + 1..at + 1 + len)
}

/// The key for a SERIAL record's bus controller, or None when it is
/// neither a PCI function nor a published platform device.
pub fn bus_key(r: &[u8], paths: &[(Vec<u8>, u64)]) -> Option<u64> {
    let adr = u64::from_le_bytes(r.get(ADR..ADR + 8)?.try_into().ok()?);
    if adr != u64::MAX {
        return Some(((adr >> 16) & 0x1F) << 3 | (adr & 7));
    }
    let ctl = string(r, 2).filter(|p| !p.is_empty())?;
    paths.iter().find(|(p, _)| p.as_slice() == ctl).map(|(_, id)| PLATFORM | id)
}
