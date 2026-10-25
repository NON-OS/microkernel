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

//! MkAcpiDeviceName as a look (mode 0): an ACPI-bus device's `_HID` by
//! broker device id. The record has no room for it, and it picks the
//! controller's register model.

use nonos_libc::mk_syscall_raw;

const DEVICE_NAME: i64 = u32::from_le_bytes(*b"MADN") as i64;
/// `_HID` (8), `_UID` (8), path length (1), path (up to 127).
const NAME_MAX: usize = 8 + 8 + 1 + 127;
const LOOK: u64 = 0;

pub fn hid_of(device_id: u64) -> Option<[u8; 8]> {
    let mut b = [0u8; NAME_MAX];
    let out = b.as_mut_ptr() as u64;
    let n = mk_syscall_raw(DEVICE_NAME, [LOOK, 0, device_id, out, 0, 0]);
    if usize::try_from(n).ok()? < 17 {
        return None;
    }
    let mut hid = [0u8; 8];
    hid.copy_from_slice(&b[..8]);
    Some(hid)
}
