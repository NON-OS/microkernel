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

use super::table::PageTable;

static mut KERNEL_L0: PageTable = PageTable::new();
static mut KERNEL_L1: PageTable = PageTable::new();
static mut KERNEL_L2: [PageTable; 6] = [
    PageTable::new(),
    PageTable::new(),
    PageTable::new(),
    PageTable::new(),
    PageTable::new(),
    PageTable::new(),
];

/// Level 1 for the high half. TTBR1 walks the same level 0 table as TTBR0, so
/// the direct map hangs off its own level 1 rather than sharing the identity
/// map's.
static mut KERNEL_L1_HIGH: PageTable = PageTable::new();
/// How many level 1 entries have level 3 tables behind them.
pub(super) const L3_L1_SPAN: usize = 4;
static mut KERNEL_L3: [[PageTable; 512]; L3_L1_SPAN] = [[PageTable::new(); 512]; L3_L1_SPAN];

/*
 * The table getters hand out `&'static mut` to a static, so two live results for
 * the same table alias. Callers must hold at most one at a time, which the boot
 * path does: it builds the tables on the boot CPU before any secondary or
 * interrupt exists. The borrow is taken through a raw pointer so no shared or
 * unique reference to the whole static is ever formed on the way.
 */

/// # Safety
/// No other reference to `KERNEL_L0` may be live while the result is.
pub(super) unsafe fn l0() -> &'static mut PageTable {
    &mut *core::ptr::addr_of_mut!(KERNEL_L0)
}

/// # Safety
/// No other reference to `KERNEL_L1` may be live while the result is.
pub(super) unsafe fn l1() -> &'static mut PageTable {
    &mut *core::ptr::addr_of_mut!(KERNEL_L1)
}

/// # Safety
/// No other reference to `KERNEL_L2[index]` may be live while the result is.
pub(super) unsafe fn l2(index: usize) -> &'static mut PageTable {
    &mut (*core::ptr::addr_of_mut!(KERNEL_L2))[index]
}

/// # Safety
/// No other reference to `KERNEL_L3[l1][l2]` may be live while the result is.
pub(super) unsafe fn l3(l1: usize, l2: usize) -> &'static mut PageTable {
    &mut (*core::ptr::addr_of_mut!(KERNEL_L3))[l1][l2]
}

pub(super) unsafe fn l0_addr() -> u64 {
    core::ptr::addr_of!(KERNEL_L0) as u64
}

pub(super) unsafe fn l1_addr() -> u64 {
    core::ptr::addr_of!(KERNEL_L1) as u64
}

/// # Safety
/// No other reference to `KERNEL_L1_HIGH` may be live while the result is.
pub(super) unsafe fn l1_high() -> &'static mut PageTable {
    &mut *core::ptr::addr_of_mut!(KERNEL_L1_HIGH)
}

pub(super) unsafe fn l1_high_addr() -> u64 {
    core::ptr::addr_of!(KERNEL_L1_HIGH) as u64
}

pub(super) unsafe fn l2_addr(index: usize) -> u64 {
    core::ptr::addr_of!((*core::ptr::addr_of!(KERNEL_L2))[index]) as u64
}

pub(super) unsafe fn l3_addr(l1: usize, l2: usize) -> u64 {
    core::ptr::addr_of!((*core::ptr::addr_of!(KERNEL_L3))[l1][l2]) as u64
}
