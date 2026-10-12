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

//! Level 2 tables handed out by the gigabyte they describe.
//!
//! A level 1 entry covers 1 GiB and a level 2 table the 512 blocks of 2 MiB
//! inside it (Arm ARM, DDI 0487, the 4 KiB granule's translation levels). So
//! whatever lands in one gigabyte, RAM or registers, has to share that one
//! table: a table per region instead maps a region only up to the end of the
//! gigabyte its base is in, and two windows in one gigabyte each overwrite the
//! other's level 1 entry. The Pi 4 has both: 3 GiB of RAM from 1 GiB up, and
//! its peripherals in the same top gigabyte as the last of that RAM.

use core::sync::atomic::{AtomicUsize, Ordering};

use super::state::PageTable;

/// Enough for 32 GiB of RAM identity and direct mapped, registers beside it.
const L2_POOL: usize = 72;
static mut KERNEL_L2: [PageTable; L2_POOL] = [PageTable::new(); L2_POOL];
static NEXT: AtomicUsize = AtomicUsize::new(0);

/// # Safety
/// No other reference to `KERNEL_L2[index]` may be live while the result is.
unsafe fn l2(index: usize) -> &'static mut PageTable {
    &mut (*core::ptr::addr_of_mut!(KERNEL_L2))[index]
}

fn l2_addr(index: usize) -> u64 {
    // SAFETY: an address, formed without a reference to the static.
    unsafe { core::ptr::addr_of!((*core::ptr::addr_of!(KERNEL_L2))[index]) as u64 }
}

/// The level 2 table under `l1[index]`, taken from the pool and installed if
/// the entry is empty. `None` when the pool is spent, or the entry is a block
/// or a table not from the pool, which nothing here may write through.
///
/// # Safety
/// Boot CPU or the kernel map's lock held; nothing else refers to `l1` or it.
pub(super) unsafe fn l2_under(l1: &mut PageTable, index: usize) -> Option<&'static mut PageTable> {
    if !l1.is_valid(index) {
        let slot = NEXT.fetch_add(1, Ordering::Relaxed);
        if slot >= L2_POOL {
            NEXT.store(L2_POOL, Ordering::Relaxed);
            return None;
        }
        l1.set_table(index, l2_addr(slot));
        return Some(l2(slot));
    }
    existing_l2(l1, index)
}

/// The pool's level 2 table under `l1[index]`, if there is one.
/// # Safety: as for [`l2_under`].
pub(super) unsafe fn existing_l2(l1: &PageTable, index: usize) -> Option<&'static mut PageTable> {
    let at = l1.table_address(index)?;
    let used = NEXT.load(Ordering::Relaxed).min(L2_POOL);
    (0..used).find(|slot| l2_addr(*slot) == at).map(|slot| l2(slot))
}
