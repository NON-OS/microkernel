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

//! The power-off wipe's pass over every live address space: each present
//! user leaf frame scrubbed in place, found by walking the page tables, so a
//! page the process maps but no VMA records (its image's data, an arena,
//! a stack) goes too. Nothing is freed or unmapped: the machine is about to
//! stop. No lock is taken and nothing is allocated here.

use super::leaf_release::{leaf, scrubbed_at_end};
use crate::memory::layout::DIRECTMAP_BASE;
use crate::memory::paging::constants::KERNEL_ASID;
use crate::memory::paging::constants::{
    pte_address, pte_is_huge, pte_is_present, PAGE_TABLE_ENTRIES,
};

fn table_at(phys: u64) -> *const [u64; PAGE_TABLE_ENTRIES] {
    (DIRECTMAP_BASE + phys) as *const [u64; PAGE_TABLE_ENTRIES]
}

// SAFETY: `table_phys` is a page-table page of a live address space, which
// the other CPUs, parked by now, no longer change.
unsafe fn scrub_subtree(table_phys: u64, level: u8, base_va: u64) {
    let table = unsafe { &*table_at(table_phys) };
    let span = 1u64 << (12 + 9 * (level as u64 - 1));
    for (i, &value) in table.iter().enumerate() {
        if !pte_is_present(value) {
            continue;
        }
        let va = base_va + i as u64 * span;
        let next = pte_address(value);
        if level == 1 || pte_is_huge(value) {
            let device = crate::hardware::broker::touches_device_window(va, span);
            if scrubbed_at_end(leaf(false, device)) {
                crate::security::secure_zero((DIRECTMAP_BASE + next) as *mut u8, span as usize);
            }
        } else {
            unsafe { scrub_subtree(next, level - 1, va) };
        }
    }
}

/// Scrub the user half, PML4[0..256], of the address space rooted at `cr3`.
pub(super) fn scrub_user_half(cr3: u64) {
    // SAFETY: as for `scrub_subtree`; the root is that space's PML4.
    let pml4 = unsafe { &*table_at(cr3) };
    for (i, &value) in pml4.iter().take(PAGE_TABLE_ENTRIES / 2).enumerate() {
        if pte_is_present(value) {
            unsafe { scrub_subtree(pte_address(value), 3, i as u64 * (1u64 << 39)) };
        }
    }
}

impl super::super::core::PagingManager {
    /// Scrub every live user address space; how many there were.
    pub fn scrub_user_spaces(&self) -> usize {
        let user = self.address_spaces.iter().filter(|(asid, _)| **asid != KERNEL_ASID);
        user.map(|(_, space)| scrub_user_half(space.cr3_value.as_u64())).count()
    }
}
