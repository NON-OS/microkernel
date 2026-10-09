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

//! Mapping 4 KiB pages in a domain. The whole range is checked before any
//! entry changes, so a refused call leaves nothing half mapped, and the
//! domain's cached translations are dropped before it returns.

use super::super::error::AmdViError;
use super::super::flush::flush_domain;
use super::super::pte::{is_present, leaf};
use super::lifetime::{root_of, ROOTS};
use super::range::{leaf_at, pages, PAGE};
use super::walk::walk_create;

pub fn map_range(
    domain: u16,
    iova: u64,
    phys: u64,
    size: usize,
    (read, write): (bool, bool),
) -> Result<(), AmdViError> {
    if !read && !write {
        return Err(AmdViError::NoPermissions);
    }
    let count = pages(iova, phys, size)?;
    let roots = ROOTS.lock();
    let root = root_of(&roots, domain)?;
    for page in 0..count {
        let (table, slot) = walk_create(root, iova + page * PAGE)?;
        if is_present(*leaf_at(table, slot)?) {
            return Err(AmdViError::RangeAlreadyMapped);
        }
    }
    for page in 0..count {
        let (table, slot) = walk_create(root, iova + page * PAGE)?;
        *leaf_at(table, slot)? = leaf(phys + page * PAGE, read, write);
    }
    flush_domain(domain)
}
