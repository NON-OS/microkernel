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

//! One 4 KiB page of an address space that is not running: its frame and
//! the permissions the live entry grants.

use super::globals::PAGING_MANAGER;
#[cfg(not(target_arch = "x86_64"))]
use crate::arch::run_without_interrupts as without_interrupts;
#[cfg(target_arch = "x86_64")]
use crate::arch::x86_64::idt::without_interrupts;
use crate::memory::addr::{PhysAddr, VirtAddr};
use crate::memory::paging::constants::{
    pte_address, pte_is_executable, pte_is_user, pte_is_writable,
};
use crate::memory::paging::types::PagePermissions;
use crate::smp::lock_responsive;

/// The frame mapped at the 4 KiB page `va` in `asid` and what its entry
/// grants, or None when no 4 KiB entry maps it.
pub fn page_in_asid(asid: u32, va: VirtAddr) -> Option<(PhysAddr, PagePermissions)> {
    let entry = without_interrupts(|| lock_responsive(&PAGING_MANAGER).leaf_in_asid(asid, va))?;
    let mut perms = PagePermissions::READ;
    if pte_is_writable(entry) {
        perms = perms.insert(PagePermissions::WRITE);
    }
    if pte_is_executable(entry) {
        perms = perms.insert(PagePermissions::EXECUTE);
    }
    if pte_is_user(entry) {
        perms = perms.insert(PagePermissions::USER);
    }
    Some((PhysAddr::new(pte_address(entry)), perms))
}
