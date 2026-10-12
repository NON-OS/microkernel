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

//! MkAcpiOscGrant: what a PCI root bridge's _OSC offered and what NONOS
//! took, for the kernel's AER and hot plug to read.

use super::acpi::{ok, tag4, Errno};
use nonos_libc::mk_syscall_raw;

const OSC_GRANT: i64 = tag4(b"MAOG");

pub fn osc_grant(segment: u16, buses: (u8, u8), offered: u32, granted: u32) -> Result<(), Errno> {
    let b = u64::from(buses.0) << 8 | u64::from(buses.1);
    let args = [u64::from(segment), b, u64::from(offered), u64::from(granted), 0, 0];
    ok(mk_syscall_raw(OSC_GRANT, args)).map(|_| ())
}
