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

//! The errno each ACPI grant failure returns to the capsule.

use crate::hardware::broker::acpi::{RegionError, TableError};
use crate::syscall::microkernel::errnos::{
    ERRNO_ACCES, ERRNO_FAULT, ERRNO_INVAL, ERRNO_IO, ERRNO_NOENT, ERRNO_NOMEM, ERRNO_NOSPC,
    ERRNO_NOTSUP,
};

pub(super) fn region_errno(e: RegionError) -> i64 {
    match e {
        RegionError::Refused(_) | RegionError::NotGranted => ERRNO_ACCES,
        RegionError::Full => ERRNO_NOSPC,
        RegionError::NoVaSpace | RegionError::MapFailed => ERRNO_NOMEM,
        RegionError::BadWidth => ERRNO_INVAL,
        RegionError::Unsupported => ERRNO_NOTSUP,
        RegionError::Platform => ERRNO_IO,
    }
}

/// A table that fails its checks is refused as a fault in the table, apart
/// from one that is not there or not the capsule's to read.
pub(super) fn table_errno(e: TableError) -> i64 {
    match e {
        TableError::Absent => ERRNO_NOENT,
        TableError::Unmapped => ERRNO_FAULT,
        TableError::Length | TableError::Signature | TableError::Checksum => ERRNO_IO,
    }
}
