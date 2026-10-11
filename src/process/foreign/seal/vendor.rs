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

//! Only a supervisor the shipped policy tree proved may close or note in a
//! family's seal. A developer-built one still gets its rows recorded, and
//! no call of its own reaches the record.

use crate::security::attest_registry::authority_of;
use crate::security::dev_roots::Authority;
use crate::syscall::microkernel::errnos::{ERRNO_INVAL, ERRNO_PERM};

pub(crate) fn vendor_caller() -> Result<u32, i64> {
    let caller = crate::process::current_pid().ok_or(ERRNO_INVAL)?;
    match authority_of(caller) {
        Some(Authority::Vendor) => Ok(caller),
        _ => Err(ERRNO_PERM),
    }
}
