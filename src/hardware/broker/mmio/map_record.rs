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

//! A finished MMIO map entered as a grant, only while its claim holds
//! (K10). A claim released since the map began finds the pages mapped and
//! no grant to revoke them by, so they come out here and the map fails.

use super::types::{MmioMapError, MmioMapRequest};
use super::window::Window;
use crate::hardware::broker::claim::record_while_held;
use crate::hardware::broker::grant::{self, MmioGrant};
use crate::memory::addr::VirtAddr;

/// The grant's id, or the refusal with the pages unmapped and their VA back.
pub(super) fn record(
    pid: u32,
    req: &MmioMapRequest,
    claim_epoch: u64,
    w: &Window,
    user_va: VirtAddr,
) -> Result<u64, MmioMapError> {
    let grant_id = grant::allocate_id();
    let entered = record_while_held(pid, req.device_id, claim_epoch, || {
        grant::insert(MmioGrant {
            grant_id,
            pid,
            device_id: req.device_id,
            claim_epoch,
            bar_index: req.bar_index,
            // The record holds the whole pages, which is what unmap gives back.
            physical_start: w.page_start,
            user_va: user_va.as_u64(),
            length: w.page_bytes,
            flags: req.flags,
        })
    });
    if entered != Some(true) {
        let _ = crate::memory::paging::unmap_user_mmio(user_va, w.page_bytes as usize);
        grant::release_user_va(pid, user_va.as_u64(), w.page_bytes);
        // No claim, or the holder's quota spent, as a spent window answers.
        return Err(entered.map_or(MmioMapError::StaleEpoch, |_| MmioMapError::NoVaSpace));
    }
    Ok(grant_id)
}
