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

//! Entering a finished map in the grant table.

use super::super::records;
use super::super::types::{DmaGrant, DmaMapRequest};
use super::placed::Placed;

pub(super) fn record(
    pid: u32,
    req: &DmaMapRequest,
    claim_epoch: u64,
    placed: &Placed,
    device_addr: u64,
    confined: bool,
) -> Option<u64> {
    let grant_id = records::allocate_id();
    let live =
        |g: &DmaGrant| crate::hardware::broker::claim::live(g.pid, g.device_id, g.claim_epoch);
    let entered = records::insert_if(
        DmaGrant {
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
        },
        live,
    );
    // Its claim gone meanwhile (L69): the caller takes the map back.
    if !entered {
        return None;
    }

    /*
     * A grant no remapping unit confines hands the device the host-physical
     * address; the posture line counts it, and only those, until released.
     */
    if !confined {
        crate::memory::iommu::note_unconfined(1);
    }
    Some(grant_id)
}
