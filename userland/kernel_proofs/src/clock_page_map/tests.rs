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

//! The page-table entry the clock page is mapped with, through the kernel's
//! own permission-to-PTE conversion: present, user, read-only, no-execute,
//! never global; and its address is a user page clear of the mmap window
//! and the stack.

use super::mapping::{PERMS, VA};
use crate::memory::paging::constants::{
    PTE_GLOBAL, PTE_NO_EXECUTE, PTE_PRESENT, PTE_USER, PTE_WRITABLE,
};
use crate::memory::paging::types::PagePermissions;

#[test]
fn the_entry_is_user_read_only_and_never_executable() {
    let pte = PERMS.to_pte_flags();
    assert_ne!(pte & PTE_PRESENT, 0, "present");
    assert_ne!(pte & PTE_USER, 0, "user");
    assert_eq!(pte & PTE_WRITABLE, 0, "never writable");
    assert_ne!(pte & PTE_NO_EXECUTE, 0, "never executable");
    assert_eq!(pte & PTE_GLOBAL, 0, "a user entry is never global");
    assert!(!PERMS.is_wx_violation());
    assert!(!PERMS.contains(PagePermissions::WRITE));
    assert!(!PERMS.contains(PagePermissions::EXECUTE));
}

#[test]
fn the_address_is_a_user_page_clear_of_mmap_and_stack() {
    const USER_MMAP_END: u64 = 0x0000_7000_0000_0000; // process/mmap_va.rs
    const STACK_BOTTOM: u64 = 0x0000_7FFF_FFFF_0000 - 2 * 1024 * 1024 - 4096; // with guard
    assert_eq!(VA % 4096, 0);
    const _: () = assert!(VA >= USER_MMAP_END);
    const _: () = assert!(VA + 4096 <= STACK_BOTTOM);
    const _: () = assert!(VA < 0x0000_8000_0000_0000);
}
