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
//! reaching here proves it. The list is device_list.rs.

#[path = "device_errno.rs"]
mod device_errno;

use super::errnos::{ERRNO_NODEV, ERRNO_PERM};
use crate::hardware::broker;
use crate::process::current_pid;

pub use super::device_list::sys_device_list;

pub fn sys_device_claim(device_id: u64) -> i64 {
    let pid = match current_pid() {
        Some(p) => p,
        None => return ERRNO_PERM,
    };
    let pid = crate::hardware::broker::holder_of(pid);
    if !broker::contains(device_id) {
        return ERRNO_NODEV;
    }
    match broker::claim_device(pid, device_id) {
        Ok(epoch) => epoch as i64,
        Err(e) => device_errno::claim_errno(e),
    }
}

// Releases the claim on `device_id` held by the calling pid, with every
// grant made against it (`give_back`, L66): retired first, so nothing more
// is granted; Bus Master off; its MMIO, IRQ, DMA and port grants taken
// down, each page out of the holder's tables and each grant out of the
// device's domain before a frame is freed (K10); the claim last, and the
// function's wipe owed then (L78). The exit path does the same for every
// claim a dying capsule held.
pub fn sys_device_release(device_id: u64) -> i64 {
    let pid = match current_pid() {
        Some(p) => p,
        None => return ERRNO_PERM,
    };
    let pid = crate::hardware::broker::holder_of(pid);
    match broker::give_back(pid, device_id) {
        Ok(_epoch) => 0,
        Err(e) => device_errno::release_errno(e),
    }
}
