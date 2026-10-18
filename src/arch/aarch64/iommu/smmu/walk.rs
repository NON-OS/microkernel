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

//! Finding the leaf slot for one IOVA in a domain's stage 1 tables.

use crate::memory::iommu::IommuError;

use super::encode::pte::{self, level_index, LEVELS};
use super::frame;

/// The level 3 entry for `iova` under the level 1 table `ttb`. With
/// `grow`, missing tables on the way are made; without, a missing one
/// means nothing is mapped there and the answer is `None`.
pub(super) fn leaf(
    ttb: u64,
    iova: u64,
    grow: bool,
) -> Result<Option<&'static mut u64>, IommuError> {
    let mut table = ttb;
    for level in 1..LEVELS {
        let entry = &mut frame::words(table)?[level_index(iova, level)];
        if !pte::is_valid(*entry) {
            if !grow {
                return Ok(None);
            }
            let next = frame::alloc()?;
            super::wait::publish();
            *entry = pte::table(next);
        }
        table = pte::address(*entry);
    }
    Ok(Some(&mut frame::words(table)?[level_index(iova, LEVELS)]))
}

/// Every table below `ttb`, deepest first, then `ttb` itself: the order
/// they can be freed in once no STE points at the domain.
pub(super) fn free_tree(ttb: u64) {
    free_level(ttb, 1);
}

fn free_level(table: u64, level: usize) {
    if level < LEVELS {
        if let Ok(words) = frame::words(table) {
            for &entry in words.iter() {
                if pte::is_valid(entry) {
                    free_level(pte::address(entry), level + 1);
                }
            }
        }
    }
    frame::free(table);
}
