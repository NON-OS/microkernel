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

//! Unmapping 4 KiB pages. The range is gone from the device's view when this
//! returns Ok: the broker frees the frames next, and a cached translation
//! would let the device write into their next owner.

use super::super::error::AmdViError;
use super::super::flush::flush_domain;
use super::super::pte::is_present;
use super::lifetime::{root_of, ROOTS};
use super::range::{leaf_at, pages, PAGE};
use super::walk::walk_lookup;

pub fn unmap_range(domain: u16, iova: u64, size: usize) -> Result<(), AmdViError> {
    let count = pages(iova, 0, size)?;
    let roots = ROOTS.lock();
    let root = root_of(&roots, domain)?;
    // A caller naming a range it does not hold does not lose the part it does.
    for page in 0..count {
        match walk_lookup(root, iova + page * PAGE)? {
            Some((table, slot)) if is_present(*leaf_at(table, slot)?) => {}
            _ => return Err(AmdViError::RangeNotMapped),
        }
    }
    for page in 0..count {
        if let Some((table, slot)) = walk_lookup(root, iova + page * PAGE)? {
            *leaf_at(table, slot)? = 0;
        }
    }
    flush_domain(domain)
}
