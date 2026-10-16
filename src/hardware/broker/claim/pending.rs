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


//! A claim between its entry in the table and its last step (L65). The entry
//! keeps every other claimant out from the start, but nothing is granted
//! against it and no sibling thread releases it: only the call that made it
//! settles it, admitted or taken back.

use super::state::CLAIMS;

fn is(pid: u32, device_id: u64, epoch: u64) -> impl Fn(&super::Claim) -> bool {
    move |c| c.pid == pid && c.device_id == device_id && c.epoch == epoch
}

/// The last step: from here the claim is granted against. The boot
/// framebuffer's hold moves under the table's lock, so a release always lands
/// after it. False when the claim is gone, its holder having exited meanwhile.
pub(super) fn admit(pid: u32, device_id: u64, epoch: u64) -> bool {
    let mut claims = CLAIMS.lock();
    let Some(claim) = claims.iter_mut().find(|c| is(pid, device_id, epoch)(c)) else {
        return false;
    };
    claim.ready = true;
    crate::hardware::broker::bootfb::claimed(device_id);
    true
}

/// A refused claim taken back by the call that made it. Nothing was granted
/// against it, so only its own steps are undone. The device leaves the
/// capsule's domain even when an exit took the entry first, since the attach
/// may have come after that exit's own detach.
pub(super) fn withdraw(pid: u32, device_id: u64, epoch: u64) {
    let mut claims = CLAIMS.lock();
    let at = claims.iter().position(is(pid, device_id, epoch));
    if let Some(at) = at {
        claims.remove(at);
    }
    drop(claims);
    // Only while it was ours: once the entry is gone another claim may hold it.
    if at.is_some() {
        super::quiesce::stop_bus_master_quietly(device_id);
    }
    crate::hardware::broker::confine::detach(pid, device_id);
}
