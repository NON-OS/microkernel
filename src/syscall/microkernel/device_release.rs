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

use core::sync::atomic::{AtomicU32, Ordering};

use super::errnos::{ERRNO_INVAL, ERRNO_NODEV, ERRNO_PERM};
use crate::hardware::broker::{self, ClaimError};
use crate::process::current_pid;

static DEVICE_RELEASE_TRACE_COUNT: AtomicU32 = AtomicU32::new(0);

fn trace_release(label: &[u8], pid: u32) {
    if pid != 7 || DEVICE_RELEASE_TRACE_COUNT.fetch_add(1, Ordering::Relaxed) >= 24 {
        return;
    }
    crate::sys::serial::print(b"[DEV-RELEASE] ");
    crate::sys::serial::println(label);
}

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
    trace_release(b"enter", pid);
    let _ = broker::irq_release_for_device(pid, device_id);
    let _ = broker::pio_release_for_device(pid, device_id);
    match broker::release_device(pid, device_id) {
        Ok(_epoch) => {
            trace_release(b"ok", pid);
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
