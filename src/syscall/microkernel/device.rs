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

//! Device enumerate / claim / release. Cap-gated at the contract
//! layer: `MkDeviceList` requires `DeviceEnum`, claim and release
//! require `Driver`. Handlers do not re-check the capability;
//! reaching here proves it. List and release: device_list.rs and
//! device_release.rs.

use super::errnos::{ERRNO_BUSY, ERRNO_INVAL, ERRNO_NODEV, ERRNO_PERM};
use crate::hardware::broker::{self, ClaimError};
use crate::process::current_pid;

pub use super::device_list::sys_device_list;
pub use super::device_release::sys_device_release;

pub fn sys_device_claim(device_id: u64) -> i64 {
    let pid = match current_pid() {
        Some(p) => p,
        None => return ERRNO_PERM,
    };
    if !broker::contains(device_id) {
        return ERRNO_NODEV;
    }
    match broker::claim_device(pid, device_id) {
        Ok(epoch) => epoch as i64,
        Err(ClaimError::AlreadyClaimed) | Err(ClaimError::Busy) => ERRNO_BUSY,
        Err(ClaimError::UnknownDevice) => ERRNO_NODEV,
        Err(ClaimError::NotHolder) | Err(ClaimError::NotClaimed) => ERRNO_INVAL,
        Err(ClaimError::Unconfined) | Err(ClaimError::OutOfScope) => ERRNO_PERM,
    }
}
