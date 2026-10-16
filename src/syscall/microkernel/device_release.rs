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

//! MkDeviceRelease (Driver, checked at the contract layer).

use super::errnos::{ERRNO_INVAL, ERRNO_NODEV, ERRNO_PERM};
use crate::hardware::broker::{self, ClaimError};
use crate::process::current_pid;

// Releases the claim on `device_id` held by the calling pid. Its IRQ and
// PIO grants go first. Then release_device takes the claim out and
// retires the device (claim/retire.rs): Bus Master off, the BARs unmapped
// and shot down, the function reset, the DMA frames freed, the domain
// detached. The exit path retires every claim a dying capsule held the
// same way.
pub fn sys_device_release(device_id: u64) -> i64 {
    let pid = match current_pid() {
        Some(p) => p,
        None => return ERRNO_PERM,
    };
    let _ = broker::irq_release_for_device(pid, device_id);
    let _ = broker::pio_release_for_device(pid, device_id);
    match broker::release_device(pid, device_id) {
        Ok(_epoch) => {
            // A bind or grant that passed its claim check before the claim
            // went, and was entered after the drops above.
            let _ = broker::irq_release_for_device(pid, device_id);
            let _ = broker::pio_release_for_device(pid, device_id);
            0
        }
        Err(ClaimError::NotClaimed) => ERRNO_NODEV,
        Err(ClaimError::NotHolder) => ERRNO_PERM,
        Err(ClaimError::AlreadyClaimed)
        | Err(ClaimError::Busy)
        | Err(ClaimError::UnknownDevice)
        | Err(ClaimError::Unconfined)
        | Err(ClaimError::OutOfScope) => ERRNO_INVAL,
    }
}
