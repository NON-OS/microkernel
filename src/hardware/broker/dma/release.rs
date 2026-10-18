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

//! DMA grant revocation. Three triggers:
//!
//!   * `MkDmaUnmap` — explicit holder request
//!   * a device retiring (claim/retire.rs) — takes its grants, frees
//!     them after the reset (`take_for_device`, `free_taken`)
//!   * process exit — drains every grant the dying pid owns
//!
//! Revocation order: unmap user pages (when the holder's CR3 is
//! active), take the grant from the device's domain, scrub, free.
//! The cross-pid path skips the user unmap because a foreign address
//! space would walk the wrong page tables; the AS reaper drops those.

use super::drain;
use super::records;
use super::teardown::{release_frames, teardown, withdraw};
use super::types::{DmaError, DmaGrant};

pub fn unmap_grant(pid: u32, grant_id: u64) -> Result<(), DmaError> {
    let g = records::remove(pid, grant_id)?;
    teardown(&g, true);
    Ok(())
}

pub fn release_all_for_pid(pid: u32, unmap_pages: bool) -> usize {
    let drained = drain::drain_for_pid(pid);
    for g in &drained {
        teardown(g, unmap_pages);
    }
    drained.len()
}

/// The device's grants out of the table, so no other path can free them,
/// while the device is reset. Freed after, by `free_taken`.
pub(crate) fn take_for_device(pid: u32, device_id: u64) -> alloc::vec::Vec<DmaGrant> {
    drain::drain_for_device(pid, device_id)
}

/// The taken grants out of their device's domain, once it is reset; keeps
/// only those no device can reach any longer (`withdraw`).
pub(crate) fn withdraw_taken(taken: &mut alloc::vec::Vec<DmaGrant>, unmap_pages: bool) {
    taken.retain(|g| withdraw(g, unmap_pages));
}

/// Frees grants `withdraw_taken` kept.
pub(crate) fn free_taken(taken: &[DmaGrant]) {
    taken.iter().for_each(release_frames);
}
