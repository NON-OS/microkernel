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

//! A claim's entry made (L65, L78), in the one hold of the claims' lock that
//! also asks the function's wipe record: no claim begins between an entry's
//! going and its wipe being owed, nor over a wipe still under way, and the
//! wait for one is the claimant's own, in user code, before any entry exists.

use super::stage_rule::over;
use super::state::{claims, next_epoch};
use super::types::{Claim, ClaimError};
use super::wipe::{self, Admit, Fresh};
use crate::kernel_core::init::framebuffer::surrendered;

/// Frames kept for a function past this refuse its claims (our bound).
const KEEP_CAP: u64 = 256 << 20;

pub(super) fn enter(pid: u32, device_id: u64, fresh: Option<&Fresh>) -> Result<u64, ClaimError> {
    wipe::step_all(true);
    let mut claims = claims();
    if let Some(c) = claims.iter().find(|c| c.device_id == device_id) {
        return Err(over(c.stage));
    }
    // Read after the record, under the claims and wipes locks (L80); the
    // kept list's lock is a leaf.
    let capped = || crate::hardware::broker::dma::kept_bytes(device_id) >= KEEP_CAP;
    match fresh.map(|f| wipe::admits(device_id, f.by, capped)) {
        None | Some(Admit::Go) => {}
        Some(Admit::Rearm) => {
            wipe::rearm(device_id);
            return Err(ClaimError::Resetting);
        }
        Some(Admit::Resetting) => return Err(ClaimError::Resetting),
        Some(Admit::Refused) => {
            drop(claims);
            crate::log::info!(
                "[BROKER] pid={} refused device {}: kept for its last holder's driver service, or \
                 past its kept frames' bound",
                pid,
                device_id
            );
            return Err(ClaimError::Refused);
        }
    }
    // Once a GPU driver's claim took its scanout, for the rest of the boot.
    if crate::hardware::broker::bootfb::is_boot_framebuffer(device_id) && surrendered() {
        return Err(ClaimError::Refused);
    }
    let epoch = next_epoch();
    claims.push(Claim::making(pid, device_id, epoch));
    Ok(epoch)
}
