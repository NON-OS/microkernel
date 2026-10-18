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

//! MkAcpiProximity: a PCI root bridge's buses and its proximity domain,
//! for the broker to answer which domain a function lies in.

use super::acpi::{ok, tag4, Errno};
use nonos_libc::mk_syscall_raw;

const PROXIMITY: i64 = tag4(b"MAPX");

pub fn proximity(segment: u16, first: u8, last: u8, domain: u32) -> Result<(), Errno> {
    let buses = u64::from(first) << 8 | u64::from(last);
    let args = [u64::from(segment), buses, u64::from(domain), 0, 0, 0];
    ok(mk_syscall_raw(PROXIMITY, args)).map(|_| ())
}
