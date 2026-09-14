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

use crate::memory::addr::VirtAddr;
use crate::memory::layout::{self, Section};
use crate::memory::paging::{self, PagePermissions};

/// Every page across `section` is mapped and matches the section exactly.
///
/// Both directions are checked, and only one of them used to be. A .text page
/// must be executable and must also not be writable, or the kernel's own code
/// is a write target. A .data page must be writable and must not be
/// executable, or the kernel carries a ready place to run injected bytes
/// from. The old form asked only whether an rx section was executable and an
/// rw section writable, so it passed a kernel mapped entirely RWX.
///
/// `nx` is the section's own statement of the second half and it was sitting
/// unread. .rodata is the case that shows why it matters: it is neither rx
/// nor rw, so under the old form neither branch fired and a writable
/// executable .rodata satisfied the check.
///
/// The whole span is walked. A single page sampled at `section.start` says
/// nothing about the mapping of the pages behind it.
/// How many kernel sections are mapped as declared, against how many there
/// are. The boot reports this, which is what keeps the section table and
/// everything reading it out of the linker's dead-code pass.
pub fn conformance() -> (usize, usize) {
    let sections = layout::kernel_sections();
    (sections.iter().filter(|s| section_conforms(s)).count(), sections.len())
}

pub(super) fn section_conforms(section: &Section) -> bool {
    let page = layout::PAGE_SIZE as u64;
    let mut va = layout::align_down(section.start, page);
    while va < section.end {
        let Some(perms) = paging::get_page_permissions(VirtAddr::new(va)) else {
            return false;
        };
        if perms.contains(PagePermissions::EXECUTE) == section.nx {
            return false;
        }
        if perms.contains(PagePermissions::WRITE) != section.rw {
            return false;
        }
        va = va.saturating_add(page);
    }
    true
}
