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

//! MkAcpiDeviceName: an ACPI-bus device's `_HID`, `_UID` and path by
//! broker device id, as a look (mode 0) or for a driver as the device's
//! holder (mode 1), the same claim test MkAcpiFunctionOf makes for PCI.

use super::acpi::{ok, tag4, Errno};
use super::function_of::Asker;
use alloc::vec::Vec;
use nonos_libc::mk_syscall_raw;

const DEVICE_NAME: i64 = tag4(b"MADN");
const NAME_MAX: usize = 8 + 8 + 1 + 127;

pub struct Name {
    pub hid: [u8; 8],
    pub uid: u64,
    /// Empty for a controller the kernel's static scan found.
    pub path: Vec<u8>,
}

pub fn device_name(asker: Asker, device_id: u64) -> Result<Name, Errno> {
    let (mode, pid) = match asker {
        Asker::Capsule => (0, 0),
        Asker::Holder(pid) => (1, u64::from(pid)),
    };
    let mut b = [0u8; NAME_MAX];
    let out = b.as_mut_ptr() as u64;
    let n = ok(mk_syscall_raw(DEVICE_NAME, [mode, pid, device_id, out, 0, 0]))? as usize;
    let b = &b[..n.min(NAME_MAX)];
    let (head, rest) = b.split_at_checked(17).ok_or(-22i64)?;
    let mut hid = [0u8; 8];
    hid.copy_from_slice(&head[..8]);
    let uid = u64::from_le_bytes(head[8..16].try_into().unwrap_or([0; 8]));
    let path = rest.get(..usize::from(head[16])).ok_or(-22i64)?.to_vec();
    Ok(Name { hid, uid, path })
}
