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
//! Revocation paths for MMIO grants: `unmap_grant` (one grant, by its
//! holder), `release_for_device` (every grant of one claim, L71) and
//! `release_all_for_pid` (what a pid still owns). Each grant's pages come
//! out of the tables its holder runs in, from any context, shot down on
//! every CPU running them; found none, the active tables are unmapped
//! when they are the holder's, and an address-space teardown drops the rest.

use super::fence::{fence, Fenced};
use crate::hardware::broker::claim;
use crate::hardware::broker::grant::{self, GrantError, MmioGrant};
use crate::hardware::broker::holder_space::unmap_from_holder;
use crate::memory::addr::VirtAddr;

pub fn unmap_grant(pid: u32, grant_id: u64) -> Result<(), GrantError> {
    // Inside its claim until the pages are gone, so a release or a takeover
    // of the scanout waits out an unmap already under way (L79).
    let _inside = grant::find(pid, grant_id).and_then(|(d, e)| claim::inside(pid, d, e));
    let g = grant::remove(pid, grant_id)?;
    unmap_one(&g);
    grant::release_user_va(pid, g.user_va, g.length);
    Ok(())
}

pub fn release_for_device(pid: u32, device_id: u64, epoch: u64) -> usize {
    let drained = grant::drain_for_device(pid, device_id, epoch);
    drained.iter().for_each(|g| take_away(g, true));
    drained.len()
}

/// An exit's MMIO drain. Its claims already left (L75), so a grant a dying
/// sibling enters after this is refused (L69) and the drain finds every one.
pub fn release_all_for_pid(pid: u32, unmap_pages: bool) -> usize {
    let drained = grant::drain_for_pid(pid);
    drained.iter().for_each(|g| take_away(g, unmap_pages));
    drained.len()
}

/// A device that left while `pid` held it: its grants are dropped without an
/// unmap, each window fenced until the holder's space goes.
pub fn revoke_for_device(pid: u32, device_id: u64, epoch: u64) -> usize {
    let drained = grant::drain_for_device(pid, device_id, epoch);
    for g in &drained {
        fence(Fenced { pid, start: g.physical_start, len: g.length });
    }
    drained.len()
}

/// Out of its holder's tables and its VA back; else out of the active ones.
fn take_away(g: &MmioGrant, active: bool) {
    if unmap_from_holder(g.pid, g.user_va, g.length) {
        grant::release_user_va(g.pid, g.user_va, g.length);
    } else if active {
        unmap_one(g);
    }
}

fn unmap_one(g: &MmioGrant) {
    let _ = crate::memory::paging::unmap_user_mmio(VirtAddr::new(g.user_va), g.length as usize);
}
