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
//! DMA grant revocation. Four triggers:
//!
//!   * `MkDmaUnmap` — explicit holder request
//!   * `MkDeviceRelease` — drains every grant made under the claim it retired
//!   * a device that left (`MkPciDepart`) — `revoke_for_device`
//!   * process exit — drains every grant the dying pid owns
//!
//! Revocation order (teardown.rs): the holder's pages unmapped from the
//! tables it runs in, from any context, else from the active tables
//! when they are its own; the grant out of the device's domain, the IOTLB
//! invalidation waited on; then scrubbed and freed, or kept for the
//! device's wipe where it may still reach the frames (L78).

use super::drain;
use super::records;
use super::teardown::teardown;
use super::types::DmaError;

pub fn unmap_grant(pid: u32, grant_id: u64) -> Result<(), DmaError> {
    let g = records::remove(pid, grant_id)?;
    teardown(&g, true, false);
    Ok(())
}

pub fn release_for_device(pid: u32, device_id: u64, epoch: u64) -> usize {
    let drained = drain::drain_for_device(pid, device_id, epoch);
    for g in &drained {
        teardown(g, true, true);
    }
    drained.len()
}

/// A device that left while `pid` held it: every grant leaves the device's
/// domain and is scrubbed; the holder's pages come out of its own tables as
/// `teardown` finds them, or with its address space.
pub fn revoke_for_device(pid: u32, device_id: u64, epoch: u64) -> usize {
    let drained = drain::drain_for_device(pid, device_id, epoch);
    for g in &drained {
        teardown(g, false, true);
    }
    drained.len()
}

pub fn release_all_for_pid(pid: u32, unmap_pages: bool) -> usize {
    let drained = drain::drain_for_pid(pid);
    for g in &drained {
        teardown(g, unmap_pages, true);
    }
    drained.len()
}
