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

//! DMA grant revocation. Three triggers:
//!
//!   * `MkDmaUnmap` — explicit holder request
//!   * `MkDeviceRelease` — drains every grant tied to the device
//!   * process exit — drains every grant the dying pid owns
//!
//! Revocation order: unmap user pages (when the holder's CR3 is
//! active), take the grant from the device's domain, scrub, free.
//! The cross-pid path skips the user unmap because a foreign address
//! space would walk the wrong page tables; the AS reaper drops those.

use super::pool;
use super::records;
use super::types::{DmaError, DmaGrant};
use crate::memory::addr::VirtAddr;
use crate::memory::phys::free_contiguous;

const PAGE_SIZE: u64 = 4096;

pub fn unmap_grant(pid: u32, grant_id: u64) -> Result<(), DmaError> {
    let g = records::remove(pid, grant_id)?;
    teardown(&g, true);
    Ok(())
}

pub fn release_for_device(pid: u32, device_id: u64) -> usize {
    let drained = records::drain_for_device(pid, device_id);
    for g in &drained {
        teardown(g, true);
    }
    drained.len()
}

pub fn release_all_for_pid(pid: u32, unmap_pages: bool) -> usize {
    let drained = records::drain_for_pid(pid);
    for g in &drained {
        teardown(g, unmap_pages);
    }
    drained.len()
}

fn teardown(g: &DmaGrant, unmap_pages: bool) {
    if unmap_pages {
        let _ = crate::memory::paging::unmap_user_dma(VirtAddr::new(g.user_va), g.length as usize);
    }
    // The device loses the grant before anyone else can gain the frames. If
    // its domain will not give it up, the frames are leaked, never reused.
    if !super::super::confine::unmap(g.pid, g.device_addr, g.length, g.confined) {
        crate::sys::serial::println(b"[DMA] grant still reachable by its device; frames quarantined");
        return;
    }
    super::scrub::scrub(g.physical_start, g.length);
    let pages = (g.length / PAGE_SIZE) as usize;
    if pool::low32_owns(g.physical_start) {
        pool::low32_free(g.physical_start, pages);
    } else if !pool::free(g.physical_start, pages) {
        let _ = free_contiguous(g.physical_start, pages);
    }
    if !g.confined {
        crate::memory::iommu::note_unconfined_released(1);
    }
}
