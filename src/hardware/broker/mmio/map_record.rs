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
//! A map's last step (L69, K10): its grant entered only while its claim
//! stands and within its holder's quota. An exit drops the claim and then
//! drains the table, so a grant entered after the drain would outlive the
//! claim; one refused has its pages taken back and its VA returned instead.

use super::types::{MmioMapError, MmioMapRequest};
use super::window::Window;
use crate::hardware::broker::grant::{self, MmioGrant};
use crate::hardware::broker::quota_log;
use crate::memory::addr::VirtAddr;

/// The grant for the pages of `w` mapped at `user_va`; its id once entered.
pub(super) fn record(
    pid: u32,
    req: &MmioMapRequest,
    epoch: u64,
    w: &Window,
    user_va: VirtAddr,
) -> Result<u64, MmioMapError> {
    let grant_id = grant::allocate_id();
    enter(MmioGrant {
        grant_id,
        pid,
        device_id: req.device_id,
        claim_epoch: epoch,
        bar_index: req.bar_index,
        // The record holds the whole pages, which is what unmap gives back.
        physical_start: w.page_start,
        user_va: user_va.as_u64(),
        length: w.page_bytes,
        flags: req.flags,
    })
    .map(|()| grant_id)
}

fn enter(g: MmioGrant) -> Result<(), MmioMapError> {
    let entered = grant::insert_live(g);
    if entered == Some(Ok(())) {
        return Ok(());
    }
    let _ = crate::memory::paging::unmap_user_mmio(VirtAddr::new(g.user_va), g.length as usize);
    grant::release_user_va(g.pid, g.user_va, g.length);
    // No claim, or the holder's quota spent, as a spent window answers.
    Err(match entered {
        Some(Err(spent)) => {
            quota_log::refused(g.pid, "MMIO", spent);
            MmioMapError::NoVaSpace
        }
        _ => MmioMapError::NotClaimed,
    })
}
