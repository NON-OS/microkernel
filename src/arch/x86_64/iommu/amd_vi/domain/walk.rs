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

//! Walking a domain's four-level table to the 4 KiB slot of an IOVA.

use super::super::error::AmdViError;
use super::super::pte::{address, directory, index, is_present, next_level, LEVELS};
use super::super::visible::store;
use crate::arch::x86_64::iommu::tables::frame::allocate_table;
use crate::arch::x86_64::iommu::tables::view::Frame;

/// The level 1 table and slot for `iova`, creating directories on the way.
/// A new table is zeroed before the directory entry naming it is written.
pub(super) fn walk_create(root: u64, iova: u64) -> Result<(u64, usize), AmdViError> {
    let mut table = root;
    for level in (2..=LEVELS).rev() {
        let entries = Frame::at(table).map_err(|_| AmdViError::TableUnreachable)?;
        let slot = index(iova, level);
        if !is_present(entries.read(slot)?) {
            let next = allocate_table().map_err(|_| AmdViError::NoFrames)?;
            store(table, slot, directory(next, level))?;
        }
        table = address(entries.read(slot)?);
    }
    Ok((table, index(iova, 1)))
}

/// As `walk_create`, but `None` where a directory is missing.
pub(super) fn walk_lookup(root: u64, iova: u64) -> Result<Option<(u64, usize)>, AmdViError> {
    let mut table = root;
    for level in (2..=LEVELS).rev() {
        let entries = Frame::at(table).map_err(|_| AmdViError::TableUnreachable)?;
        let entry = entries.read(index(iova, level))?;
        if !is_present(entry) {
            return Ok(None);
        }
        if next_level(entry) != level - 1 {
            return Err(AmdViError::TableUnreachable);
        }
        table = address(entry);
    }
    Ok(Some((table, index(iova, 1))))
}
