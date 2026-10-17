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

//! MMIO grant table.
//!
//! Records every page-aligned physical range the broker has mapped
//! into a capsule address space. The table is the authority on which
//! pid owns which user MMIO window; revocation paths walk it on
//! `MkDeviceRelease` and on process exit, unmap the user pages, and
//! broadcast TLB invalidations through the per-asid shootdown path.
//!
//! The kernel exposes no other physical-mapping primitive to
//! userland. Bypassing this table means inventing a new one — the
//! static checks make that loud.

extern crate alloc;

use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, Ordering};
use spin::Mutex;

#[derive(Debug, Clone, Copy)]
pub struct MmioGrant {
    pub grant_id: u64,
    pub pid: u32,
    pub device_id: u64,
    pub claim_epoch: u64,
    pub bar_index: u8,
    pub physical_start: u64,
    pub user_va: u64,
    pub length: u64,
    pub flags: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GrantError {
    UnknownGrant,
    NotHolder,
}

static GRANTS: Mutex<Vec<MmioGrant>> = Mutex::new(Vec::new());
static NEXT_GRANT_ID: AtomicU64 = AtomicU64::new(1);

fn next_grant_id() -> u64 {
    NEXT_GRANT_ID.fetch_add(1, Ordering::SeqCst)
}

// Insert a new grant, while its claim stands (L69). The caller (broker
// MMIO map handler) has already validated claim ownership, BAR containment
// and alignment, and installed the user pages; false when the claim is gone,
// for the caller to take the pages back.
pub(super) fn insert_live(record: MmioGrant) -> bool {
    let mut grants = GRANTS.lock();
    let live = super::claim::live(record.pid, record.device_id, record.claim_epoch);
    if live {
        grants.push(record);
    }
    live
}

// Allocate a fresh grant id without recording yet. Lets the handler
// stamp the id into the record before insertion.
pub(super) fn allocate_id() -> u64 {
    next_grant_id()
}

// Revoke every grant held by `pid`. Returns the list of revoked
// records so the caller can decide whether to unmap user pages
// (self-context) or skip the unmap and let address-space teardown
// drop the PTEs (cross-pid).
pub(super) fn drain_for_pid(pid: u32) -> Vec<MmioGrant> {
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
pub(super) fn drain_for_device(pid: u32, device_id: u64, epoch: u64) -> Vec<MmioGrant> {
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

// The device and claim of `pid`'s grant `grant_id`, as recorded, so an
// unmap can hold that claim while it takes the pages back (L79).
pub(super) fn find(pid: u32, grant_id: u64) -> Option<(u64, u64)> {
    let grants = GRANTS.lock();
    let g = grants.iter().find(|g| g.grant_id == grant_id && g.pid == pid)?;
    Some((g.device_id, g.claim_epoch))
}

// Remove a single grant if `pid` is the holder. Returns the removed
// record on success.
pub(super) fn remove(pid: u32, grant_id: u64) -> Result<MmioGrant, GrantError> {
    let mut grants = GRANTS.lock();
    let idx = grants.iter().position(|g| g.grant_id == grant_id).ok_or(GrantError::UnknownGrant)?;
    if grants[idx].pid != pid {
        return Err(GrantError::NotHolder);
    }
    Ok(grants.remove(idx))
}

#[path = "grant_va.rs"]
mod va;
pub(super) use va::{reserve_user_va, USER_MMIO_BASE, USER_MMIO_END};
