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

//! Capsule domains on AMD-Vi: a domain id and the root of its page table.
//! Id 0 is never handed out, so a zeroed device table entry never names a
//! live domain.

use spin::Mutex;

use super::super::error::AmdViError;
use super::super::flush::flush_domain;
use super::super::pte::LEVELS;
use super::super::pte_child::child_table;
use crate::arch::x86_64::iommu::tables::frame::allocate_table;
use crate::arch::x86_64::iommu::tables::free::all_readable;

pub const MAX_DOMAINS: usize = 256;

/// Root of each domain's page table, zero while the id is free.
pub(super) static ROOTS: Mutex<[u64; MAX_DOMAINS]> = Mutex::new([0; MAX_DOMAINS]);
/// An id whose tables could not all go back: taken, never walked, never
/// reused. Not page aligned, so it can never be a real root.
const QUARANTINED: u64 = 1;

/// The lowest free id, with a fresh zeroed root table.
pub fn create_domain() -> Result<u16, AmdViError> {
    let mut roots = ROOTS.lock();
    let id = (1..MAX_DOMAINS).find(|&i| roots[i] == 0).ok_or(AmdViError::DomainTableFull)?;
    roots[id] = allocate_table().map_err(|_| AmdViError::NoFrames)?;
    Ok(id as u16)
}

/// Free the id. Devices must be detached already (a domain a failed attach
/// may still name is pinned, quarantined). The cached translations go first,
/// so no walk is in the tables when they go back, as on VT-d. A table
/// that cannot be read keeps them all; any failure quarantines the id.
pub fn destroy_domain(id: u16) -> Result<(), AmdViError> {
    let mut roots = ROOTS.lock();
    let slot = roots.get_mut(id as usize).ok_or(AmdViError::DomainNotFound)?;
    if *slot == 0 || *slot == QUARANTINED || id == 0 {
        return Err(AmdViError::DomainNotFound);
    }
    flush_domain(id)?;
    // Never walked again, whatever happens next: a release that fails part
    // way has already freed some of these frames.
    let root = core::mem::replace(slot, QUARANTINED);
    if super::pinned::pinned(id) || !all_readable(root, LEVELS, &child_table) {
        return Err(AmdViError::TableNotFreed);
    }
    super::teardown::release_tables(root)?;
    *slot = 0;
    Ok(())
}

/// The root of a live domain, read under the caller's lock.
pub(super) fn root_of(roots: &[u64; MAX_DOMAINS], id: u16) -> Result<u64, AmdViError> {
    match roots.get(id as usize) {
        Some(&root) if root != 0 && root != QUARANTINED && id != 0 => Ok(root),
        _ => Err(AmdViError::DomainNotFound),
    }
}
