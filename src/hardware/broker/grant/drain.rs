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

//! Grants out of the table: a holder's, or a claim's (one.rs: one by id).

extern crate alloc;

use alloc::vec::Vec;

use super::table::GRANTS;
use super::types::MmioGrant;

// Revoke every grant held by `pid`. Returns the list of revoked
// records so the caller can decide whether to unmap user pages
// (self-context) or skip the unmap and let address-space teardown
// drop the PTEs (cross-pid).
pub(in super::super) fn drain_for_pid(pid: u32) -> Vec<MmioGrant> {
    let mut grants = GRANTS.lock();
    let mut taken = Vec::new();
    grants.retain(|g| {
        if g.pid == pid {
            taken.push(*g);
            false
        } else {
            true
        }
    });
    taken
}

// Revoke every grant made under `pid`'s claim of `device_id` at `epoch`,
// when the holder gives it back: a newer claim's are not this release's
// to take (L71).
pub(in super::super) fn drain_for_device(pid: u32, device_id: u64, epoch: u64) -> Vec<MmioGrant> {
    let mut grants = GRANTS.lock();
    let mut taken = Vec::new();
    grants.retain(|g| {
        if (g.pid, g.device_id, g.claim_epoch) == (pid, device_id, epoch) {
            taken.push(*g);
            false
        } else {
            true
        }
    });
    taken
}
