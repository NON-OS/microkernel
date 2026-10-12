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

//! Level 3 tables for the identity map's low gigabytes, one per 2 MiB: where
//! the image is described page by page and where `map_page` puts a page.

use super::table::PageTable;

/// How many level 1 entries have level 3 tables behind them.
pub(in crate::arch::aarch64::mmu) const L3_L1_SPAN: usize = 4;
static mut KERNEL_L3: [[PageTable; 512]; L3_L1_SPAN] = [[PageTable::new(); 512]; L3_L1_SPAN];

/// # Safety
/// No other reference to `KERNEL_L3[l1][l2]` may be live while the result is.
pub(in crate::arch::aarch64::mmu) unsafe fn l3(l1: usize, l2: usize) -> &'static mut PageTable {
    &mut (*core::ptr::addr_of_mut!(KERNEL_L3))[l1][l2]
}

pub(in crate::arch::aarch64::mmu) unsafe fn l3_addr(l1: usize, l2: usize) -> u64 {
    core::ptr::addr_of!((*core::ptr::addr_of!(KERNEL_L3))[l1][l2]) as u64
}
