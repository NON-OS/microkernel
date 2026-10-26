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

//! A span of a guest's pages given new permissions with one TLB round
//! (span_in_asid.rs says why one).

#[cfg(not(target_arch = "x86_64"))]
use crate::arch::run_without_interrupts as without_interrupts;
#[cfg(target_arch = "x86_64")]
use crate::arch::x86_64::idt::without_interrupts;
use crate::memory::addr::VirtAddr;
use crate::memory::paging::manager::shootdown::flush_tlb_range_smp;
use crate::memory::paging::types::{PagePermissions, PageSize};
use crate::smp::lock_responsive;

use super::globals::{PAGING_MANAGER, PAGING_STATS};
use super::span_in_asid::page;

/// Give each of the `pages` pages from `start` in `asid` `permissions`,
/// keeping its frame. False at the first page that is not mapped or will
/// not take them; the pages changed before it are flushed either way.
pub fn protect_span_in_asid(
    asid: u32,
    start: VirtAddr,
    pages: usize,
    permissions: PagePermissions,
) -> bool {
    let mut changed = 0;
    let whole = (0..pages).all(|i| {
        let va = page(start, i);
        let done = without_interrupts(|| {
            let mut m = lock_responsive(&PAGING_MANAGER);
            let phys = m.translate_in_asid(asid, va)?;
            m.map_page_in_asid(asid, va, phys, permissions, PageSize::Size4KiB, &PAGING_STATS).ok()
        });
        changed += usize::from(done.is_some());
        done.is_some()
    });
    if changed > 0 {
        flush_tlb_range_smp(start, pages, asid);
    }
    whole
}
