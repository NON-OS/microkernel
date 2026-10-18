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

pub(super) use super::l3_tables::{l3, l3_addr, L3_L1_SPAN};
pub(super) use super::table::PageTable;

static mut KERNEL_L0: PageTable = PageTable::new();
static mut KERNEL_L1: PageTable = PageTable::new();

/// Level 1 for the high half. TTBR1 walks the same level 0 table as TTBR0, so
/// the direct map hangs off its own level 1 rather than sharing the identity
/// map's.
static mut KERNEL_L1_HIGH: PageTable = PageTable::new();

/*
 * The `_addr` getters answer with physical addresses: they go into
 * descriptors and TTBRs, which do not translate, and the kernel runs at its
 * high alias once it has moved.
 *
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

pub(super) unsafe fn l0_addr() -> u64 {
    super::image_pa(core::ptr::addr_of!(KERNEL_L0) as u64)
}

pub(super) unsafe fn l1_addr() -> u64 {
    super::image_pa(core::ptr::addr_of!(KERNEL_L1) as u64)
}

/// # Safety
/// No other reference to `KERNEL_L1_HIGH` may be live while the result is.
pub(super) unsafe fn l1_high() -> &'static mut PageTable {
    &mut *core::ptr::addr_of_mut!(KERNEL_L1_HIGH)
}

pub(super) unsafe fn l1_high_addr() -> u64 {
    super::image_pa(core::ptr::addr_of!(KERNEL_L1_HIGH) as u64)
}
