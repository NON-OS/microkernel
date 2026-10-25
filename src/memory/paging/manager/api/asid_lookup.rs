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

//! An address space found by its page tables rather than its owner: a
//! thread runs in its process's tables and owns no entry of its own.

use super::globals::PAGING_MANAGER;
use crate::smp::lock_responsive;

/// The asid whose tables are `cr3`, if one is registered.
pub fn lookup_asid_for_cr3(cr3: u64) -> Option<u32> {
    let mgr = lock_responsive(&PAGING_MANAGER);
    mgr.address_spaces
        .iter()
        .find(|(_, space)| space.cr3_value.as_u64() == cr3)
        .map(|(asid, _)| *asid)
}
