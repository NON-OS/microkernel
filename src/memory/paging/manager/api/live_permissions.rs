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

//! Permissions as the hardware holds them, not as the manager remembers them.

use super::globals::PAGING_MANAGER;
use crate::arch::run_without_interrupts as without_interrupts;
use crate::memory::addr::VirtAddr;
use crate::memory::paging::constants::{pte_is_executable, pte_is_user, pte_is_writable};
use crate::memory::paging::types::PagePermissions;

/// What the live page tables grant at `virtual_addr`, or nothing when no
/// entry maps it.
///
/// `get_page_permissions` answers from the manager's record of its own
/// mappings and so answers nothing for every page the bootloader installed,
/// the kernel image included. Use this for any question about a mapping the
/// manager did not make.
pub fn live_page_permissions(virtual_addr: VirtAddr) -> Option<PagePermissions> {
    let entry = without_interrupts(|| PAGING_MANAGER.lock().leaf_entry(virtual_addr)).ok()?;
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
    Some(perms)
}
