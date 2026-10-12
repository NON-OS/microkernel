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

//! MkAcpiDeviceName as a look (mode 0): an ACPI-bus device's `_HID`, `_UID`
//! and namespace path by broker device id. The record has no room for
//! them; the `_HID` picks the controller model, the path names a bus
//! controller that has no PCI function.

use alloc::vec::Vec;
use nonos_libc::mk_syscall_raw;

const DEVICE_NAME: i64 = u32::from_le_bytes(*b"MADN") as i64;
const NAME_MAX: usize = 8 + 8 + 1 + 127;
const LOOK: u64 = 0;

pub struct Name {
    pub hid: [u8; 8],
    pub path: Vec<u8>,
}

pub fn name(device_id: u64) -> Option<Name> {
    let mut b = [0u8; NAME_MAX];
    let out = b.as_mut_ptr() as u64;
    let n = mk_syscall_raw(DEVICE_NAME, [LOOK, 0, device_id, out, 0, 0]);
    let b = b.get(..usize::try_from(n).ok()?.min(NAME_MAX))?;
    let (head, rest) = b.split_at_checked(17)?;
    let mut hid = [0u8; 8];
    hid.copy_from_slice(&head[..8]);
    let path = rest.get(..usize::from(head[16]))?.to_vec();
    Some(Name { hid, path })
}
