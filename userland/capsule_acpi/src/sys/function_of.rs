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

//! MkAcpiFunctionOf: the PCI function behind a broker device id, asked as
//! the capsule's own look (mode 0) or for a driver as the device's holder
//! (mode 1), which the kernel answers only if that pid holds the claim.

use super::acpi::{ok, tag4, Errno};
use nonos_libc::mk_syscall_raw;

const FUNCTION_OF: i64 = tag4(b"MAFO");

#[derive(Clone, Copy)]
pub enum Asker {
    /// The capsule's own look, for reading a device's state.
    Capsule,
    /// A driver asking to move its device.
    Holder(u32),
}

pub fn function_of(asker: Asker, device_id: u64) -> Result<(u16, u8, u8, u8), Errno> {
    let (mode, pid) = match asker {
        Asker::Capsule => (0, 0),
        Asker::Holder(pid) => (1, u64::from(pid)),
    };
    let v = ok(mk_syscall_raw(FUNCTION_OF, [mode, pid, device_id, 0, 0, 0]))?;
    let devfn = (v & 0xFF) as u8;
    Ok(((v >> 16) as u16, (v >> 8) as u8, devfn >> 3, devfn & 7))
}
