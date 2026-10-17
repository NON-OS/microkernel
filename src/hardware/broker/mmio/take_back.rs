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

//! A holder's MMIO grants under one claim taken back from outside it (L79):
//! drained from the table, then each page still showing its grant's frame
//! pointed at the sink in the holder's own tables, which a thread of it may
//! be running on another CPU. Its stores land in the sink, unseen, and its
//! loads read only what it wrote there; nothing faults.

use crate::hardware::broker::grant;
use crate::memory::addr::{PhysAddr, VirtAddr};
use crate::memory::paging::{pages_needed, repoint_user_mmio};

/// The pages moved. None when the holder's tables are gone: no thread runs
/// on them then, and their teardown never frees a page of the device window.
pub(in crate::hardware::broker) fn take_back(
    holder: u32,
    device_id: u64,
    epoch: u64,
    sink: PhysAddr,
) -> usize {
    let drained = grant::drain_for_device(holder, device_id, epoch);
    let Some(asid) = crate::kernel_core::surface_registry::space_of(holder) else {
        return 0;
    };
    let moved = |g: &grant::MmioGrant| {
        let (va, from) = (VirtAddr::new(g.user_va), PhysAddr::new(g.physical_start));
        repoint_user_mmio(asid, va, pages_needed(g.length as usize), from, sink)
    };
    drained.iter().map(moved).sum()
}
