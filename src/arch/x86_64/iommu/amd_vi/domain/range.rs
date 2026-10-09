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

//! What a map or unmap request is allowed to name, and the leaf slot it
//! edits.

use super::super::error::AmdViError;
use super::super::pte::{reach, LEVELS};
use crate::arch::x86_64::iommu::tables::frame::entries_mut;

pub(super) const PAGE: u64 = 4096;

/// Pages in a request whose IOVA, physical address and size are all 4 KiB
/// aligned and whose end the four-level table reaches.
pub(super) fn pages(iova: u64, phys: u64, size: usize) -> Result<u64, AmdViError> {
    let size = size as u64;
    if size == 0 || (iova | phys | size) & (PAGE - 1) != 0 {
        return Err(AmdViError::Misaligned);
    }
    match iova.checked_add(size) {
        Some(end) if end <= reach(LEVELS) => Ok(size / PAGE),
        _ => Err(AmdViError::OutOfRange),
    }
}

/// The leaf entry at `slot` of the level 1 table at `table`.
pub(super) fn leaf_at(table: u64, slot: usize) -> Result<&'static mut u64, AmdViError> {
    let entries = entries_mut(table).map_err(|_| AmdViError::TableUnreachable)?;
    entries.get_mut(slot).ok_or(AmdViError::OutOfRange)
}
