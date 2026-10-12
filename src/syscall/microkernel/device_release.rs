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

// Releases the claim on `device_id` held by the calling pid. Bus
// mastering stops first, so nothing the device still has in flight can
// land in a DMA frame once it is freed below; then the outstanding MMIO,
// IRQ, DMA and PIO grants are torn down (the caller's CR3 is active here
// so the unmap and TLB shootdown are in-context), and the claim goes
// last. The exit path performs the same cleanup for every claim a dying
// capsule was holding, and drops its claims before its DMA grants.
pub fn sys_device_release(device_id: u64) -> i64 {
    let pid = match current_pid() {
        Some(p) => p,
        None => return ERRNO_PERM,
    };
    trace_release(b"enter", pid);
    let _ = broker::quiesce_held_device(pid, device_id);
    let _ = broker::release_for_device(pid, device_id);
    let _ = broker::irq_release_for_device(pid, device_id);
    let _ = broker::dma_release_for_device(pid, device_id);
    let _ = broker::pio_release_for_device(pid, device_id);
    match broker::release_device(pid, device_id) {
        Ok(_epoch) => {
            trace_release(b"ok", pid);
            0
        }
        Err(ClaimError::NotClaimed) => ERRNO_NODEV,
        Err(ClaimError::NotHolder) => ERRNO_PERM,
        Err(ClaimError::AlreadyClaimed)
        | Err(ClaimError::UnknownDevice)
        | Err(ClaimError::Unconfined) => ERRNO_INVAL,
    }
}
