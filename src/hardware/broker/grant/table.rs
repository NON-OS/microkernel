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

use super::types::MmioGrant;

pub(super) static GRANTS: Mutex<Vec<MmioGrant>> = Mutex::new(Vec::new());
static NEXT_GRANT_ID: AtomicU64 = AtomicU64::new(1);

fn next_grant_id() -> u64 {
    NEXT_GRANT_ID.fetch_add(1, Ordering::SeqCst)
}

// Insert a new grant. The caller (broker MMIO map handler) has
// already validated claim ownership, BAR containment and alignment,
// and installed the user pages. This function only persists the
// record so revocation can find it later.
pub(in super::super) fn insert(record: MmioGrant) {
    GRANTS.lock().push(record);
}

// Allocate a fresh grant id without recording yet. Lets the handler
// stamp the id into the record before insertion.
pub(in super::super) fn allocate_id() -> u64 {
    next_grant_id()
}
