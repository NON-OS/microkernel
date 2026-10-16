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

//! MkPciDepart: take a function out of the kernel's device table. `Gone`
//! for one that has left (its holder loses everything it was given);
//! `Eject` before firmware's `_EJ0`, which the kernel refuses (EBUSY)
//! while anyone holds the function.

use super::acpi::{ok, tag4, Errno};
use nonos_libc::mk_syscall_raw;

const DEPART: i64 = tag4(b"MPDP");
pub const EBUSY: Errno = -16;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Why {
    Gone = 0,
    Eject = 1,
}

/// An ACPI platform record, by its device id (modes 2 and 3).
pub fn depart_record(device_id: u64, why: Why) -> Result<u64, Errno> {
    ok(mk_syscall_raw(DEPART, [device_id, 0, why as u64 + 2, 0, 0, 0]))
}

/// Segment 0 only; the device id the function had.
pub fn depart(bus: u8, dev: u8, func: u8, why: Why) -> Result<u64, Errno> {
    let devfn = u64::from(dev) << 3 | u64::from(func);
    ok(mk_syscall_raw(DEPART, [u64::from(bus), devfn, why as u64, 0, 0, 0]))
}
