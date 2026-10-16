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

use super::state::CLAIMS;
use super::types::{ClaimError, Stage};

// Release the claim `pid` retired at `epoch` (L66), and that one only: a
// sibling's release racing this one may have taken it, and the device been
// claimed again since, a newer claim this release never drained (L69).
pub fn release(pid: u32, device_id: u64, epoch: u64) -> Result<u64, ClaimError> {
    let mut claims = CLAIMS.lock();
    let ours = |c: &super::Claim| (c.pid, c.device_id, c.epoch) == (pid, device_id, epoch);
    let at = claims.iter().position(|c| ours(c) && c.stage == Stage::Leaving);
    claims.remove(at.ok_or(ClaimError::NotClaimed)?);
    drop(claims);
    super::quiesce::stop_bus_master(device_id);
    crate::hardware::broker::confine::detach(pid, device_id);
    crate::hardware::broker::bootfb::released(device_id);
    Ok(epoch)
}

/// Stop a device the caller holds from mastering the bus, before its grants
/// are torn down. False, and nothing written, when `pid` does not hold it at
/// `epoch`: a newer claim's device is not this release's to stop (L71).
pub fn quiesce_held(pid: u32, device_id: u64, epoch: u64) -> bool {
    let key = (pid, device_id, epoch);
    let held = CLAIMS.lock().iter().any(|c| (c.pid, c.device_id, c.epoch) == key);
    if held {
        super::quiesce::stop_bus_master_quietly(device_id);
    }
    held
}

// Release every claim held by `pid`. Called from the kernel's
// `MkExit` path so a dying capsule cannot leak grants. Returns the
// number of claims revoked.
pub fn release_all_for_pid(pid: u32) -> usize {
    let mut claims = CLAIMS.lock();
    let held: alloc::vec::Vec<u64> =
        claims.iter().filter(|c| c.pid == pid).map(|c| c.device_id).collect();
    // A claim its call is still making is orphaned, not removed: the device
    // stays out of other claims until that call takes it back (L66).
    for c in claims.iter_mut().filter(|c| c.pid == pid && c.stage == Stage::Making) {
        c.stage = Stage::Orphaned;
    }
    claims.retain(|c| c.pid != pid || c.stage == Stage::Orphaned);
    drop(claims);
    for device_id in &held {
        super::quiesce::stop_bus_master(*device_id);
        crate::hardware::broker::bootfb::released(*device_id);
    }
    crate::hardware::broker::confine::detach_all(pid);
    held.len()
}
