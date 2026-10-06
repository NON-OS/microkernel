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

use super::broadcast::broadcast;
use crate::memory::addr::VirtAddr;
use crate::memory::paging::constants::PAGE_SIZE_4K;
use crate::memory::paging::tlb;
use crate::smp::cpus_online;

#[inline]
pub fn flush_tlb_one_smp(va: VirtAddr, asid: u32) {
    tlb::invalidate_page(va);
    if cpus_online() <= 1 {
        return;
    }
    broadcast(va, 1, asid);
}

#[inline]
pub fn flush_tlb_range_smp(start: VirtAddr, page_count: usize, asid: u32) {
    if page_count == 0 {
        return;
    }
    if page_count > 32 {
        flush_tlb_all_smp(asid);
        return;
    }
    for i in 0..page_count {
        let va = VirtAddr::new(start.as_u64() + (i * PAGE_SIZE_4K) as u64);
        tlb::invalidate_page(va);
    }
    if cpus_online() <= 1 {
        return;
    }
    broadcast(start, page_count as u32, asid);
}

#[inline]
pub fn flush_tlb_all_smp(asid: u32) {
    tlb::invalidate_all();
    if cpus_online() <= 1 {
        return;
    }
    // Encode "flush whole TLB" as page_count == 0 in the request
    // slot; the IPI handler treats that as `invalidate_all`.
    broadcast(VirtAddr::new(0), 0, asid);
}
