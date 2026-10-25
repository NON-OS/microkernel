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

//! `MkHvBackdoor`. One VMware backdoor call from the allowed set, with the
//! four registers it returns written to the caller as `[u32; 4]`.

use core::mem::size_of;

use crate::arch::{hv_backdoor, HvBackdoorError};
use crate::syscall::microkernel::errnos::{ERRNO_FAULT, ERRNO_INVAL, ERRNO_NODEV};
use crate::usercopy::{validate_user_write, write_user_value};

pub fn sys_hv_backdoor(cmd: u64, arg: u64, out: u64) -> i64 {
    if out == 0 || validate_user_write(out, size_of::<[u32; 4]>()).is_err() {
        return ERRNO_FAULT;
    }
    let (Ok(cmd), Ok(arg)) = (u32::try_from(cmd), u32::try_from(arg)) else {
        return ERRNO_INVAL;
    };
    let regs = match hv_backdoor(cmd, arg) {
        Ok(r) => r,
        Err(HvBackdoorError::NoHost) => return ERRNO_NODEV,
        Err(HvBackdoorError::Refused) => return ERRNO_INVAL,
    };
    if write_user_value(out, &regs).is_err() {
        return ERRNO_FAULT;
    }
    0
}
