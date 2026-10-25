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

//! One grant's revocation, in the order `release.rs` describes.

use super::types::DmaGrant;
use crate::memory::addr::VirtAddr;

/// `released`: drained by a release or an exit, not unmapped by its holder.
pub(super) fn teardown(g: &DmaGrant, unmap_pages: bool, released: bool) {
    // From the holder's tables, from any context: tables that outlive
    // it, a thread's or an heir's, keep no page onto the frames given back.
    if super::super::holder_space::unmap_from_holder(g.pid, g.user_va, g.length) {
        super::va::release(g.pid, g.user_va, g.length);
    } else if unmap_pages {
        let _ = crate::memory::paging::unmap_user_dma(VirtAddr::new(g.user_va), g.length as usize);
    }
    // The device loses the grant before anyone else can gain the frames. If
    // its domain will not give it up, the frames are leaked, never reused.
    if !super::super::confine::unmap(g.pid, g.device_addr, g.length, g.confined) {
        crate::sys::serial::println(
            b"[DMA] grant still reachable by its device; frames quarantined",
        );
        return;
    }
    // Its device may still reach it past Bus Master: kept for its wipe (L78).
    let pci = crate::hardware::broker::pci_index::lookup(g.device_id).is_some();
    // A VMBus channel's page the host may still hold a GPADL over: kept until
    // the host says it tore the last one down (`vmbus::settle_channel`).
    let host = crate::hardware::broker::vmbus::host_may_reach(g.device_id);
    // Its entry is made before the registry is asked, so a let-go landing on
    // another CPU in between finds it and waits instead of freeing the run.
    if super::kept_rule::keeps(pci, g.confined, released) || host {
        super::scrub::scrub(g.physical_start, g.length);
        super::kept::keep(g.device_id, g.claim_epoch, g.physical_start, g.length);
        let run = (g.physical_start, g.length);
        if !crate::kernel_core::surface_registry::pin::grant_run::hold(run.0, run.1) {
            let _ = super::kept::defer(run.0, run.1);
        }
        return;
    }
    if !g.confined {
        crate::memory::iommu::note_unconfined_released(1);
    }
    // A surface another process still maps keeps the frames; the registry
    // frees them through `free_held_run` once it is let go (L35).
    if crate::kernel_core::surface_registry::pin::grant_run::hold(g.physical_start, g.length) {
        return;
    }
    super::held_run::free_held_run(g.physical_start, g.length);
}
