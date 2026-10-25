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

use super::types::MmioMapError;
use crate::hardware::broker::grant::{self, MmioGrant};
use crate::hardware::broker::quota_log;
use crate::memory::addr::VirtAddr;

pub(super) fn record(g: MmioGrant) -> Result<(), MmioMapError> {
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
