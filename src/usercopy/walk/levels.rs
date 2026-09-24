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

//! Four-level page-table walker.

use super::bounds::leaf_in_directmap;
use super::leaf::UserLeaf;
use super::root::page_table_root;
use super::step::{
    block, entry_at, through, IDX, PAGE_1G_MASK, PAGE_1G_SIZE, PAGE_2M_MASK, PAGE_2M_SIZE,
    PAGE_4K_MASK, PAGE_4K_SIZE,
};
use crate::arch::paging::descriptor;
use crate::usercopy::error::UsercopyError;

// Internal entry: the page-table walk only.
pub(super) fn walk_to_leaf(va: u64) -> Result<UserLeaf, UsercopyError> {
    let pt_root = page_table_root()?;
    if pt_root == 0 || pt_root & PAGE_4K_MASK != 0 {
        return Err(UsercopyError::PageTableCorrupt);
    }
    walk(pt_root, va)
}

fn walk(pt_root: u64, va: u64) -> Result<UserLeaf, UsercopyError> {
    let e4 = entry_at(pt_root, (va >> 39) & IDX)?;
    let e3 = entry_at(through(e4)?, (va >> 30) & IDX)?;
    if descriptor::is_block(e3) {
        return block(e3, va, PAGE_1G_MASK, PAGE_1G_SIZE);
    }
    let e2 = entry_at(through(e3)?, (va >> 21) & IDX)?;
    if descriptor::is_block(e2) {
        return block(e2, va, PAGE_2M_MASK, PAGE_2M_SIZE);
    }
    let e1 = entry_at(through(e2)?, (va >> 12) & IDX)?;
    leaf_in_directmap(UserLeaf {
        entry: e1,
        phys_base: descriptor::address(e1),
        offset: va & PAGE_4K_MASK,
        size: PAGE_4K_SIZE,
    })
}
