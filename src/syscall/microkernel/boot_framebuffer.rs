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

//! `MkBootFramebuffer`: the boot framebuffer's geometry, for the process
//! holding its claim only (L31). The window itself comes from the device
//! record and `MkMmioMap`; this answers what the record cannot carry: width,
//! height, row pitch, pixel format, the first pixel's offset and the panel's
//! size. Cap-gated on `Mmio` at the contract layer.

use super::errnos::{ERRNO_FAULT, ERRNO_INVAL, ERRNO_NODEV, ERRNO_PERM};
use crate::hardware::broker::{self, BootFbInfoError};
use crate::process::current_pid;
use crate::usercopy::{validate_user_write, write_user_value};

const INFO_LEN: u64 = 24;

pub fn sys_boot_framebuffer(device_id: u64, out: u64, out_len: u64) -> i64 {
    let Some(pid) = current_pid() else {
        return ERRNO_PERM;
    };
    if out == 0 || out_len < INFO_LEN {
        return ERRNO_INVAL;
    }
    let info = match broker::boot_framebuffer_info(pid, device_id) {
        Ok(bytes) => bytes,
        Err(BootFbInfoError::NotBootFramebuffer) => return ERRNO_NODEV,
        Err(BootFbInfoError::NotHolder) => return ERRNO_PERM,
    };
    if validate_user_write(out, info.len()).is_err() || write_user_value(out, &info).is_err() {
        return ERRNO_FAULT;
    }
    INFO_LEN as i64
}
