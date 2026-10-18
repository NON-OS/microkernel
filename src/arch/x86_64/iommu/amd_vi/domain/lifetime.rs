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
use super::super::pte::{address, is_present, next_level, LEVELS};
use crate::arch::x86_64::iommu::tables::frame::allocate_table;
use crate::arch::x86_64::iommu::tables::free::free_tables;

pub const MAX_DOMAINS: usize = 256;

/// Root of each domain's page table, zero while the id is free.
pub(super) static ROOTS: Mutex<[u64; MAX_DOMAINS]> = Mutex::new([0; MAX_DOMAINS]);

/// The lowest free id, with a fresh zeroed root table.
pub fn create_domain() -> Result<u16, AmdViError> {
    let mut roots = ROOTS.lock();
    let id = (1..MAX_DOMAINS).find(|&i| roots[i] == 0).ok_or(AmdViError::DomainTableFull)?;
    roots[id] = allocate_table().map_err(|_| AmdViError::NoFrames)?;
    Ok(id as u16)
}

/// Free the id. Devices must already be detached; the cached translations
/// are dropped first so the next owner of the id inherits none of them, and
/// only then do the tables go back to the frame allocator (K12): no walk
/// lands in a reused frame.
pub fn destroy_domain(id: u16) -> Result<(), AmdViError> {
    let mut roots = ROOTS.lock();
    let slot = roots.get_mut(id as usize).ok_or(AmdViError::DomainNotFound)?;
    if *slot == 0 || id == 0 {
        return Err(AmdViError::DomainNotFound);
    }
    flush_domain(id)?;
    if !free_tables(core::mem::take(slot), LEVELS, table_below) {
        crate::sys::serial::println(b"[AMD-Vi] a destroyed domain's tables kept: unreadable");
    }
    Ok(())
}

/// The table a directory entry at `level` points to, one level down.
fn table_below(entry: u64, level: u8) -> Option<u64> {
    let directory = level > 1 && is_present(entry) && next_level(entry) == level - 1;
    directory.then_some(address(entry))
}

/// The root of a live domain, read under the caller's lock.
pub(super) fn root_of(roots: &[u64; MAX_DOMAINS], id: u16) -> Result<u64, AmdViError> {
    match roots.get(id as usize) {
        Some(&root) if root != 0 && id != 0 => Ok(root),
        _ => Err(AmdViError::DomainNotFound),
    }
}
