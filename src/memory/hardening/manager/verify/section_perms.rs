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

//! Whether a kernel section's pages carry the permissions its own descriptor
//! declares, and nothing beyond them.

use super::section_fault::{Granted, SectionFault};
use crate::memory::addr::VirtAddr;
use crate::memory::layout::{self, Section};
use crate::memory::paging::{self, PagePermissions};

/// The first page across `section` that is not mapped exactly as declared.
///
/// Both directions are checked, and only one of them used to be. A .text page
/// must be executable and must also not be writable, or the kernel's own code
/// is a write target. A .data page must be writable and must not be
/// executable, or the kernel carries a ready place to run injected bytes
/// from. The old form asked only whether an rx section was executable and an
/// rw section writable, so it passed a kernel mapped entirely RWX.
///
/// `nx` is the section's own statement of the second half and it was sitting
/// unread. .rodata is the case that shows why: it is neither rx nor rw, so
/// under the old form neither branch fired and a writable executable .rodata
/// satisfied the check.
///
/// The whole span is walked. A single page sampled at `section.start` says
/// nothing about the pages behind it.
pub fn first_fault(section: &Section) -> Option<SectionFault> {
    let page = layout::PAGE_SIZE as u64;
    let mut va = layout::align_down(section.start, page);
    while va < section.end {
        let fault = SectionFault {
            va,
            granted: None,
            want_writable: section.rw,
            want_executable: !section.nx,
        };
        /*
         * The live tables, not the manager's record of what it mapped. The
         * kernel image came from the bootloader, so that record holds nothing
         * for any of these pages and get_page_permissions answers None for
         * every one: this check read 0 of 4 sections that way, on a kernel
         * whose every segment the bootloader maps correctly.
         */
        let Some(perms) = paging::live_page_permissions(VirtAddr::new(va)) else {
            return Some(fault);
        };
        let granted = Granted {
            writable: perms.contains(PagePermissions::WRITE),
            executable: perms.contains(PagePermissions::EXECUTE),
        };
        if granted.writable != fault.want_writable || granted.executable != fault.want_executable {
            return Some(SectionFault { granted: Some(granted), ..fault });
        }
        va = va.saturating_add(page);
    }
    None
}

pub(super) fn section_conforms(section: &Section) -> bool {
    first_fault(section).is_none()
}
