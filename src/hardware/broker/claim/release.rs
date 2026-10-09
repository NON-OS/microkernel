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
use super::types::ClaimError;

// Release a claim held by `pid`. Returns the released epoch on
// success.
pub fn release(pid: u32, device_id: u64) -> Result<u64, ClaimError> {
    let mut claims = CLAIMS.lock();
    let idx = claims.iter().position(|c| c.device_id == device_id).ok_or(ClaimError::NotClaimed)?;
    if claims[idx].pid != pid {
        return Err(ClaimError::NotHolder);
    }
    let epoch = claims[idx].epoch;
    claims.remove(idx);
    drop(claims);
    super::quiesce::stop_bus_master(device_id);
    crate::hardware::broker::confine::detach(pid, device_id);
    Ok(epoch)
}

/// Stop a device the caller holds from mastering the bus, before its grants
/// are torn down. False, and nothing written, when `pid` does not hold it.
pub fn quiesce_held(pid: u32, device_id: u64) -> bool {
    let held = CLAIMS.lock().iter().any(|c| c.device_id == device_id && c.pid == pid);
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
    claims.retain(|c| c.pid != pid);
    drop(claims);
    for device_id in &held {
        super::quiesce::stop_bus_master(*device_id);
    }
    crate::hardware::broker::confine::detach_all(pid);
    held.len()
}

/// Stop every claimed device from mastering the bus, for the shutdown wipe:
/// a device still writing would put bytes back behind it. The claim table is
/// only tried, since a CPU stopped by IPI may hold it; the claims themselves
/// are left in place. The number of devices stopped, or `None` if the table
/// was held.
pub fn quiesce_all() -> Option<usize> {
    let held: alloc::vec::Vec<u64> = {
        let claims = CLAIMS.try_lock()?;
        claims.iter().map(|c| c.device_id).collect()
    };
    for device_id in &held {
        super::quiesce::stop_bus_master_quietly(*device_id);
    }
    Some(held.len())
}
