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

use crate::memory::VirtAddr;

use super::super::consts::PAGE_SIZE;

/// Whether any of the `pages` pages from `base` is mapped already. A range
/// can be a gigabyte, walked with interrupts masked, so TLB shootdowns are
/// answered between pages; a walk's answer holds no translation across one.
pub(crate) fn any_mapped(base: u64, pages: u64) -> bool {
    (0..pages as usize).any(|i| {
        crate::smp::serve_shootdowns();
        crate::memory::paging::is_mapped(VirtAddr::new(base + (i * PAGE_SIZE) as u64))
    })
}
