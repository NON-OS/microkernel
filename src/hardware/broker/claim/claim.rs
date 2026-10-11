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

use super::state::{next_epoch, CLAIMS};
use super::types::{Claim, ClaimError};

// Register a claim. Returns the granted epoch on success. The caller
// must have already verified that `device_id` exists in the broker
// table.
/*
 * The claim is published last (K5, A10 gap 4): until it is in CLAIMS no
 * epoch exists, so no thread of the caller can reach the device's config
 * space or set Bus Master before it is confined, powered and snooping.
 */
pub fn claim(pid: u32, device_id: u64) -> Result<u64, ClaimError> {
    if CLAIMS.lock().iter().any(|c| c.device_id == device_id) {
        return Err(ClaimError::AlreadyClaimed);
    }
    // Confined before it is powered, so the device never runs unconfined.
    if crate::hardware::broker::confine::attach(pid, device_id).is_err() {
        return Err(ClaimError::Unconfined);
    }
    // Bus Master off: the driver turns it on itself, once it holds Dma (K6).
    super::quiesce::stop_bus_master_quietly(device_id);
    // Bring the device to power state D0 before its driver maps MMIO. Done
    // outside the claims lock: it touches config space and settles for a moment.
    crate::hardware::broker::power::power_on_device(device_id);
    // Before the driver can turn bus mastering on, so no request it ever
    // makes is no-snoop.
    super::no_snoop::snoop_every_request(device_id);
    let mut claims = CLAIMS.lock();
    // Another claim won the device meanwhile: this one's attach is undone.
    if claims.iter().any(|c| c.device_id == device_id) {
        drop(claims);
        crate::hardware::broker::confine::detach(pid, device_id);
        return Err(ClaimError::AlreadyClaimed);
    }
    let epoch = next_epoch();
    claims.push(Claim { pid, device_id, epoch });
    Ok(epoch)
}
