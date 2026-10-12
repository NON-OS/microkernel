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

//! `MkNumaHmat(level << 8 | kind, initiator, target) -> value`: HMAT's
//! latency (kind 0 access, 1 read, 2 write; picoseconds) or bandwidth (3,
//! 4, 5; MB/s) from an initiator domain to a target domain, at a memory
//! hierarchy level (0 memory, 1 to 3 the caches). For anyone who may list
//! devices, as MkNumaDistance. EINVAL for a kind or level out of range;
//! ENOENT where HMAT gives no value; ERANGE when the target cannot be
//! reached from the initiator.

use crate::hardware::broker::acpi::numa_store::{hmat, HmatValue};
use crate::syscall::microkernel::errnos::{ERRNO_INVAL, ERRNO_NOENT, ERRNO_RANGE};

pub fn sys_numa_hmat(which: u64, initiator: u64, target: u64) -> i64 {
    let (kind, level) = ((which & 0xFF) as u8, which >> 8);
    if kind > 5 || level > 3 {
        return ERRNO_INVAL;
    }
    let (Ok(from), Ok(to)) = (u32::try_from(initiator), u32::try_from(target)) else {
        return ERRNO_NOENT;
    };
    match hmat(level as u8, kind, from, to) {
        Some(HmatValue::Value(v)) => i64::try_from(v).unwrap_or(i64::MAX),
        Some(HmatValue::Unreachable) => ERRNO_RANGE,
        None => ERRNO_NOENT,
    }
}
