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

//! Unmapping, as all or nothing as mapping: a range with any page not
//! mapped is refused whole.

use crate::memory::iommu::IommuError;

use super::encode::pte;
use super::pgtable::pages;
use super::walk::leaf;

/// Clear the leaves of a range that is entirely mapped. The caller
/// invalidates the unit's TLB before reporting the range unmapped.
pub(super) fn unmap(ttb: u64, iova: u64, size: u64) -> Result<(), IommuError> {
    for page in pages(iova, size)? {
        if !leaf(ttb, page, false)?.is_some_and(|e| pte::is_valid(*e)) {
            return Err(IommuError::NotMapped);
        }
    }
    clear(ttb, iova, size)
}

pub(super) fn clear(ttb: u64, iova: u64, size: u64) -> Result<(), IommuError> {
    if size == 0 {
        return Ok(());
    }
    for page in pages(iova, size)? {
        if let Some(slot) = leaf(ttb, page, false)? {
            *slot = 0;
        }
    }
    super::wait::publish();
    Ok(())
}
