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

//! Directmap bounds for the page walk.

use super::leaf::UserLeaf;
use crate::memory::layout::{DIRECTMAP_BASE, DIRECTMAP_SIZE};
use crate::usercopy::error::UsercopyError;

/// A page table is 512 entries of eight bytes, and the walker reads
/// any of them.
const TABLE_BYTES: u64 = 4096;

/// The directmap address of a table page.
pub(super) fn table_in_directmap(phys: u64) -> Result<u64, UsercopyError> {
    fits(phys, TABLE_BYTES)?;
    Ok(DIRECTMAP_BASE + phys)
}

/// A leaf frame, whose size the walk has already decided.
pub(super) fn leaf_in_directmap(leaf: UserLeaf) -> Result<UserLeaf, UsercopyError> {
    fits(leaf.phys_base, leaf.size)?;
    Ok(leaf)
}

/// Whether `[phys, phys + len)` lies inside the window.
fn fits(phys: u64, len: u64) -> Result<(), UsercopyError> {
    match phys.checked_add(len) {
        Some(end) if end <= DIRECTMAP_SIZE => Ok(()),
        _ => Err(UsercopyError::PageTableCorrupt),
    }
}
