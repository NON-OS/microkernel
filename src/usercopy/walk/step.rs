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

//! One level of the walk.

use super::bounds::{leaf_in_directmap, table_in_directmap};
use super::leaf::UserLeaf;
use crate::arch::paging::descriptor;
use crate::usercopy::error::UsercopyError;

pub(super) const IDX: u64 = 0x1FF;
pub(super) const PAGE_4K_MASK: u64 = 0xFFF;
pub(super) const PAGE_2M_MASK: u64 = (1 << 21) - 1;
pub(super) const PAGE_1G_MASK: u64 = (1 << 30) - 1;
pub(super) const PAGE_4K_SIZE: u64 = 1 << 12;
pub(super) const PAGE_2M_SIZE: u64 = 1 << 21;
pub(super) const PAGE_1G_SIZE: u64 = 1 << 30;

/// The entry at `index` of the table at `phys`, refusing an absent one.
pub(super) fn entry_at(phys: u64, index: u64) -> Result<u64, UsercopyError> {
    let table = table_in_directmap(phys)?;
    match read_pte(table, index) {
        e if descriptor::is_present(e) => Ok(e),
        _ => Err(UsercopyError::PageNotMapped),
    }
}

/// The table below `entry`, if `entry` goes on granting user access.
pub(super) fn through(entry: u64) -> Result<u64, UsercopyError> {
    match descriptor::table_grants_user(entry) {
        true => Ok(descriptor::address(entry)),
        false => Err(UsercopyError::PageNotUser),
    }
}

/// A block descriptor as a leaf.
pub(super) fn block(entry: u64, va: u64, mask: u64, size: u64) -> Result<UserLeaf, UsercopyError> {
    leaf_in_directmap(UserLeaf {
        entry,
        phys_base: entry & descriptor::ADDR_MASK & !mask,
        offset: va & mask,
        size,
    })
}

fn read_pte(table_virt: u64, index: u64) -> u64 {
    let ptr = (table_virt + index * 8) as *const u64;
    /*
     * SAFETY: ek@nonos.systems - `table_virt` bounds a whole table
     * page in the directmap and `index` is masked to 0..=511.
     */
    unsafe { core::ptr::read_volatile(ptr) }
}
