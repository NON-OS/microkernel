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

//! Mapping 4 KiB pages in a domain's stage 1 tables, all or nothing: every
//! page is checked before any is changed.

use crate::memory::iommu::IommuError;

use super::encode::pte;
use super::walk::leaf;

const PAGE: u64 = 0x1000;

pub(super) fn pages(iova: u64, size: u64) -> Result<impl Iterator<Item = u64>, IommuError> {
    if iova % PAGE != 0 || size == 0 || size % PAGE != 0 || !pte::in_range(iova, size) {
        return Err(IommuError::InvalidSize);
    }
    Ok((0..size / PAGE).map(move |n| iova + n * PAGE))
}

pub(super) fn map(
    ttb: u64,
    iova: u64,
    phys: u64,
    size: u64,
    read: bool,
    write: bool,
) -> Result<(), IommuError> {
    if phys % PAGE != 0 {
        return Err(IommuError::InvalidSize);
    }
    if pte::page(phys, read, write).is_none() {
        return Err(IommuError::InvalidIova);
    }
    for page in pages(iova, size)? {
        if leaf(ttb, page, false)?.is_some_and(|e| pte::is_valid(*e)) {
            return Err(IommuError::AlreadyMapped);
        }
    }
    for (n, page) in pages(iova, size)?.enumerate() {
        let slot = match leaf(ttb, page, true) {
            Ok(Some(slot)) => slot,
            // Out of table frames part way: take back what was written.
            failed => {
                let _ = super::pgtable_unmap::clear(ttb, iova, n as u64 * PAGE);
                return Err(failed.err().unwrap_or(IommuError::PageTableExhausted));
            }
        };
        *slot = pte::page(phys + (page - iova), read, write).unwrap_or(0);
    }
    super::wait::publish();
    Ok(())
}
