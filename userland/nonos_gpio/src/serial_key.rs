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

//! A SERIAL record's bus-controller key (`device_key` says the forms).

use super::device_key::{pci_key, PLATFORM};
use alloc::vec::Vec;

/// SERIAL's fields (capsule_acpi op 9): the bus controller's `_ADR`, and
/// where its strings start (device path, `_HID`, bus controller path,
/// GPIO controller path, a length byte and bytes each).
const ADR: usize = 18;
const STRINGS: usize = 42;
const NO_ADR: u64 = u64::MAX;

/// The `nth` of SERIAL's length-prefixed strings.
pub fn serial_string(r: &[u8], nth: usize) -> Option<&[u8]> {
    let mut at = STRINGS;
    for _ in 0..nth {
        at = at.checked_add(1 + usize::from(*r.get(at)?))?;
    }
    let len = usize::from(*r.get(at)?);
    r.get(at + 1..at + 1 + len)
}

/// The key for a SERIAL record's bus controller: its PCI function when it
/// has an `_ADR`, else the device id of the ACPI record whose path is the
/// bus controller's (`paths`: each ACPI record's path and device id).
/// None for a controller that is neither.
pub fn bus_key(r: &[u8], paths: &[(Vec<u8>, u64)]) -> Option<u64> {
    let adr = u64::from_le_bytes(r.get(ADR..ADR + 8)?.try_into().ok()?);
    if adr != NO_ADR {
        return Some(pci_key(adr));
    }
    let ctl = serial_string(r, 2).filter(|p| !p.is_empty())?;
    let id = paths.iter().find(|(p, _)| p.as_slice() == ctl)?.1;
    (id & PLATFORM == 0).then_some(PLATFORM | id)
}
