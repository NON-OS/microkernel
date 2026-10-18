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

//! QUIRKS (16, device id u64): the machine quirks (`nonos_smbios::quirk`)
//! that apply to the device the asking driver holds, on this machine. Who
//! holds the device is the kernel's claim test, as for DEVICE_POWER; the
//! device is known by its PCI vendor and device ids, or its `_HID`. The
//! body: a count u8, then each quirk's name (length u8, bytes) and value
//! u32. No SMBIOS, or a table with nothing for this machine, is a count
//! of 0. EPERM when the sender does not hold the device; ENOENT for one
//! that is neither a PCI function nor named; EIO when its ids cannot be
//! read.

use crate::boot::Acpi;
use crate::server::protocol::{EINVAL, EIO, ENOENT, EPERM};
use crate::sys::acpi_pci::pci_read;
use crate::sys::device_name::device_name;
use crate::sys::function_of::{function_of, Asker};
use alloc::vec::Vec;
use nonos_smbios::quirk::{lookup, Device, QUIRKS};

const EPERM_RAW: i64 = EPERM as i64;

pub fn quirks(acpi: &Acpi, req: &[u8], sender: u32) -> (i32, Vec<u8>) {
    let Some(id) = req.get(1..9).map(|b| u64::from_le_bytes(b.try_into().unwrap_or([0; 8]))) else {
        return (EINVAL, Vec::new());
    };
    let asker = Asker::Holder(sender);
    let hid;
    let device = match function_of(asker, id) {
        Ok(f) => match pci_read(f, 0, 4) {
            Ok(v) => Device::Pci { vendor: v as u16, device: (v >> 16) as u16 },
            Err(_) => return (EIO, Vec::new()),
        },
        Err(EPERM_RAW) => return (EPERM, Vec::new()),
        Err(_) => match device_name(asker, id) {
            Ok(n) => {
                hid = n.hid;
                Device::Acpi { hid: hid.split(|c| *c == 0).next().unwrap_or(&[]) }
            }
            Err(EPERM_RAW) => return (EPERM, Vec::new()),
            Err(_) => return (ENOENT, Vec::new()),
        },
    };
    let found = acpi.smbios.as_ref().map(|i| lookup(QUIRKS, i, &device)).unwrap_or_default();
    let mut b = alloc::vec![found.len().min(255) as u8];
    for q in found.iter().take(255) {
        let name = &q.name.as_bytes()[..q.name.len().min(255)];
        b.push(name.len() as u8);
        b.extend_from_slice(name);
        b.extend_from_slice(&q.value.to_le_bytes());
    }
    (0, b)
}
