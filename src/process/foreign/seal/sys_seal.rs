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

//! `MkForeignSeal`: a vendor supervisor seals its family once no guest is
//! left.

use super::finish::finish;
use super::vendor::vendor_caller;
use crate::syscall::microkernel::errnos::{ERRNO_BUSY, ERRNO_NOENT};

/// The seal's handle, or EBUSY while a guest lives: a family sealed early
/// would leave the rest of its run in no record at all.
pub(crate) fn sys_foreign_seal() -> i64 {
    let caller = match vendor_caller() {
        Ok(c) => c,
        Err(e) => return e,
    };
    if super::super::registry::guest_count(caller) != 0 {
        return ERRNO_BUSY;
    }
    match finish(caller, true) {
        Some(n) => n as i64,
        None => ERRNO_NOENT,
    }
}
