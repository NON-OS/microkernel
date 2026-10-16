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

//! A SERIAL record's bus-controller key (capsule_acpi op 9): after its four
//! strings (device path, `_HID`, bus controller path, GPIO controller path,
//! a length byte and bytes each) comes the key, u64 little-endian, all ones
//! when capsule_acpi found no device in the table for the controller.

use super::device_key::device_of;

const STRINGS: usize = 42;

/// The `nth` of SERIAL's length-prefixed strings, and where it ends.
fn string(r: &[u8], nth: usize) -> Option<(&[u8], usize)> {
    let mut at = STRINGS;
    for _ in 0..nth {
        at = at.checked_add(1 + usize::from(*r.get(at)?))?;
    }
    let end = at + 1 + usize::from(*r.get(at)?);
    Some((r.get(at + 1..end)?, end))
}

pub fn serial_string(r: &[u8], nth: usize) -> Option<&[u8]> {
    string(r, nth).map(|(s, _)| s)
}

/// All ones: capsule_acpi found no device in the table for the controller.
const NONE: u64 = u64::MAX;

/// None when the record is cut short or names no controller in the table.
pub fn bus_key(r: &[u8]) -> Option<u64> {
    let (_, end) = string(r, 3)?;
    let key = u64::from_le_bytes(r.get(end..end + 8)?.try_into().ok()?);
    (key != NONE).then_some(key).filter(|k| device_of(*k).is_some())
}
