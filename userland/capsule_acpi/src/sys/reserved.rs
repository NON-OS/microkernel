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

//! MkAcpiReserved: the motherboard resource ranges, for the broker to keep
//! out of every grant.

use super::acpi::{ok, tag4, Errno};
use nonos_libc::mk_syscall_raw;

const RESERVED: i64 = tag4(b"MAMR");

/// How many ranges the broker keeps in all.
pub fn reserved(ranges: &[u8]) -> Result<u64, Errno> {
    let (ptr, len) = (ranges.as_ptr() as u64, ranges.len() as u64);
    ok(mk_syscall_raw(RESERVED, [ptr, len, 0, 0, 0, 0]))
}
