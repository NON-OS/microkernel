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

//! The kernel image's frames, for forbidden_ranges.rs: read from the page
//! tables, so a bootloader that placed the image anywhere is still covered.

extern crate alloc;

use alloc::vec::Vec;

use crate::memory::addr::VirtAddr;

const PAGE: u64 = 4096;

/// The kernel image's pages, by their mapping, merged into runs.
pub(super) fn push(out: &mut Vec<(u64, u64)>) {
    let (start, end) = (crate::memory::layout::kernel_start(), crate::memory::layout::kernel_end());
    let mut va = start & !(PAGE - 1);
    while va < end {
        if let Some(pa) = crate::memory::paging::translate_address(VirtAddr::new(va)) {
            let pa = pa.as_u64() & !(PAGE - 1);
            match out.last_mut() {
                Some(run) if run.1 == pa => run.1 = pa + PAGE,
                _ => out.push((pa, pa + PAGE)),
            }
        }
        va += PAGE;
    }
}
