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

//! MMIO grant table: every page-aligned physical range the broker has mapped
//! into a capsule, and the authority on which pid owns which user MMIO
//! window; revocation walks it at `MkDeviceRelease` and at exit. The kernel
//! gives userland no other physical-mapping primitive; bypassing this table
//! means inventing a new one, which the static checks make loud.

extern crate alloc;

use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, Ordering};
use spin::Mutex;

use super::super::quota;
use super::types::MmioGrant;

pub(super) static GRANTS: Mutex<Vec<MmioGrant>> = Mutex::new(Vec::new());
static NEXT_GRANT_ID: AtomicU64 = AtomicU64::new(1);

fn next_grant_id() -> u64 {
    NEXT_GRANT_ID.fetch_add(1, Ordering::SeqCst)
}

// Insert a new grant, while its claim stands (L69) and within its holder's
// quota, both asked under the table's lock. The caller has validated
// the claim, the BAR and the alignment, and mapped the pages, which it
// unmaps again on a refusal: None when the claim is gone.
pub(in super::super) fn insert_live(record: MmioGrant) -> Option<Result<(), quota::Spent>> {
    let mut grants = GRANTS.lock();
    if !super::super::claim::live(record.pid, record.device_id, record.claim_epoch) {
        return None;
    }
    Some(push(&mut grants, record))
}

fn push(grants: &mut Vec<MmioGrant>, record: MmioGrant) -> Result<(), quota::Spent> {
    let held = grants.iter().filter(|g| g.pid == record.pid).count();
    if let Some(spent) = quota::spent(held, 0, 0, quota::MAX_MMIO_GRANTS, u64::MAX) {
        return Err(spent);
    }
    grants.push(record);
    Ok(())
}

// Allocate a fresh grant id without recording yet. Lets the handler
// stamp the id into the record before insertion.
pub(in super::super) fn allocate_id() -> u64 {
    next_grant_id()
}

crate::arch::acpi_only! {
    mod unclaimed;
    pub(in crate::hardware::broker) use unclaimed::insert_unclaimed;
}
