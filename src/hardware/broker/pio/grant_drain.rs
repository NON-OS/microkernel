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

//! Revocation of PIO grants in bulk: every grant a device's claimant
//! holds when it lets the device go, every grant a capsule holds when it
//! exits.

extern crate alloc;

use alloc::vec::Vec;

use super::grant::{PioGrant, GRANTS};

pub(super) fn drain_for_pid(pid: u32) -> Vec<PioGrant> {
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

/// The grants of `pid`'s claim of `device_id` at `epoch` only (L71).
pub(super) fn drain_for_device(pid: u32, device_id: u64, epoch: u64) -> Vec<PioGrant> {
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
