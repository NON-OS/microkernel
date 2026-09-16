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

use super::bits::*;

#[inline]
pub const fn is_present(entry: u64) -> bool {
    entry & VALID != 0
}

/// True when this entry maps a block rather than pointing at another table.
/// Only meaningful at levels 1 and 2, which is where the manager asks.
#[inline]
pub const fn is_block(entry: u64) -> bool {
    entry & VALID != 0 && entry & TABLE_OR_PAGE == 0
}

#[inline]
pub const fn address(entry: u64) -> u64 {
    entry & ADDR_MASK
}

#[inline]
pub const fn is_writable(entry: u64) -> bool {
    entry & AP_READ_ONLY == 0
}

#[inline]
pub const fn is_user(entry: u64) -> bool {
    entry & AP_EL0 != 0
}

/// True when instructions may be fetched from a leaf this entry maps, by the
/// exception level the mapping belongs to.
///
/// The two execute-never bits are asymmetric, which is why this cannot be a
/// single mask. A user mapping runs at EL0 and is executable when UXN is
/// clear; a kernel mapping runs at EL1 and is executable when PXN is clear.
/// `leaf` sets the opposite bit in each case, so the level that does not own
/// the mapping never gets to run from it.
#[inline]
pub const fn is_executable(entry: u64) -> bool {
    if entry & VALID == 0 {
        return false;
    }
    if entry & AP_EL0 != 0 {
        entry & UXN == 0
    } else {
        entry & PXN == 0
    }
}

/// Unlike x86_64, a table descriptor restricts nothing unless its hierarchical
/// bits say so, and the ones this kernel writes leave them clear.
#[inline]
pub const fn table_grants_user(entry: u64) -> bool {
    entry & APTABLE_NO_EL0 == 0
}
