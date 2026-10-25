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
//! Entering a finished map in the grant table, only while its claim stands
//! (L69) and within its holder's quota; on a refusal the caller undoes
//! the map.

use super::super::records;
use super::super::types::{DmaGrant, DmaMapError, DmaMapRequest};
use super::placed::Placed;
use crate::hardware::broker::quota_log;

pub(super) fn record(
    pid: u32,
    req: &DmaMapRequest,
    claim_epoch: u64,
    placed: &Placed,
    device_addr: u64,
    confined: bool,
) -> Result<u64, DmaMapError> {
    let grant_id = records::allocate_id();
    let live =
        |g: &DmaGrant| crate::hardware::broker::claim::live(g.pid, g.device_id, g.claim_epoch);
    let grant = DmaGrant {
        grant_id,
        pid,
        device_id: req.device_id,
        claim_epoch,
        physical_start: placed.phys_start,
        user_va: placed.user_va,
        length: req.length,
        flags: req.flags,
        device_addr,
        confined,
    };
    match records::insert_if(grant, live) {
        // Its claim gone meanwhile (L69): the caller takes the map back.
        None => return Err(DmaMapError::NotClaimed),
        // The quota spent answers as a spent window does, and is named.
        Some(Err(spent)) => {
            quota_log::refused(pid, "DMA", spent);
            return Err(DmaMapError::NoVaSpace);
        }
        Some(Ok(())) => {}
    }

    /*
     * A grant no remapping unit confines hands the device the host-physical
     * address; the posture line counts it, and only those, until released.
     */
    if !confined {
        crate::memory::iommu::note_unconfined(1);
    }
    Ok(grant_id)
}
