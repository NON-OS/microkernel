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

//! A span of a guest's pages unmapped or reprotected with one TLB round.
//!
//! A page at a time, each change waited for every CPU running the address
//! space to acknowledge its own shootdown: a 1 MiB peer call was 256 rounds,
//! all under the peer lock, while every other supervisor waited on it. Each
//! page is still changed under the paging lock; the invalidation owed for
//! the span is paid once after the last, before any frame goes back.

use alloc::vec::Vec;

use super::globals::{PAGING_MANAGER, PAGING_STATS};
#[cfg(not(target_arch = "x86_64"))]
use crate::arch::run_without_interrupts as without_interrupts;
#[cfg(target_arch = "x86_64")]
use crate::arch::x86_64::idt::without_interrupts;
use crate::memory::addr::{PhysAddr, VirtAddr};
use crate::memory::paging::constants::PAGE_SIZE_4K;
use crate::memory::paging::manager::shootdown::flush_tlb_range_smp;
use crate::memory::paging::types::{PagePermissions, PageSize};
use crate::smp::lock_responsive;

pub(super) fn page(start: VirtAddr, i: usize) -> VirtAddr {
    VirtAddr::new(start.as_u64() + (i * PAGE_SIZE_4K) as u64)
}

/// Drop the `pages` pages from `start` that are mapped in `asid`, and hand
/// back their frames once no CPU can still reach them.
pub fn unmap_span_in_asid(
    asid: u32,
    start: VirtAddr,
    pages: usize,
    permissions: PagePermissions,
) -> Vec<PhysAddr> {
    let mut frames = Vec::new();
    for i in 0..pages {
        let removed = without_interrupts(|| {
            let mut m = lock_responsive(&PAGING_MANAGER);
            m.unmap_page_in_asid(
                asid,
                page(start, i),
                permissions,
                PageSize::Size4KiB,
                &PAGING_STATS,
            )
        });
        // The span's round below covers what each page owes.
        if let Ok((frame, _owed)) = removed {
            frames.push(frame);
        }
    }
    if !frames.is_empty() {
        flush_tlb_range_smp(start, pages, asid);
    }
    frames
}
