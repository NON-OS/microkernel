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

extern crate alloc;

use super::leaving::{take_all, take_held};
use super::retire::retire;
use super::retire_io::Leaving;
use super::state::{BUSY, CLAIMS};
use super::types::ClaimError;

// Release a claim held by `pid`, the caller in its own MkDeviceRelease.
// The claim goes first, so a thread of `pid` that faults on a revoked BAR
// and tears down meanwhile finds nothing to reset or free (K10b). Returns
// the released epoch.
pub fn release(pid: u32, device_id: u64) -> Result<u64, ClaimError> {
    let epoch = {
        let mut claims = CLAIMS.lock();
        take_held(&mut claims, &mut BUSY.lock(), pid, device_id)?
    };
    retire(&mut Leaving::new(pid, device_id, Some(pid), true));
    Ok(epoch)
}

// Release every claim held by `pid`, from exit teardown, whose waits spin
// (reset_wait). `in_context` when `pid`'s page tables are active. Returns
// the number of claims revoked.
pub fn release_all_for_pid(pid: u32, in_context: bool) -> usize {
    let held = {
        let mut claims = CLAIMS.lock();
        take_all(&mut claims, &mut BUSY.lock(), pid)
    };
    for device_id in &held {
        retire(&mut Leaving::new(pid, *device_id, None, in_context));
    }
    crate::hardware::broker::confine::detach_all(pid);
    held.len()
}
