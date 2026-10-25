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

//! DMA runs a surface is registered over (L35). A GPU driver's scanout is
//! its own DMA grant, registered as the primary surface the compositor
//! maps. When the grant goes, the device loses it at once, but its frames
//! wait for the registry while another process still maps them.

use super::records::RECORDS;

/// The run of the grant `holder` holds that covers [`va`, `va + len`).
pub fn grant_run_covering(holder: u32, va: u64, len: u64) -> Option<(u64, u64)> {
    let all = RECORDS.lock();
    let mine = all.iter().filter(|g| g.pid == holder);
    super::covering::covering(mine.map(|g| (g.user_va, g.length, g.physical_start)), va, len)
}

/// Scrub a run and give it back where it came from: its pool, or the
/// allocator. The device lost it when its grant was torn down; a run its
/// device's wipe still keeps waits for that wipe (L78).
pub fn free_held_run(physical_start: u64, length: u64) {
    let kept = super::kept::defer(physical_start, length);
    if kept == Some(true) {
        return;
    }
    // A kept run is an unconfined grant (`keeps`) counted until it is freed;
    // its teardown left the count to whoever frees it (L80).
    if kept == Some(false) {
        crate::memory::iommu::note_unconfined_released(1);
    }
    free_now(physical_start, length);
}

pub(super) fn free_now(physical_start: u64, length: u64) {
    super::scrub::scrub(physical_start, length);
    let pages = (length / 4096) as usize;
    if !super::pool::give_back(physical_start, pages) {
        let _ = crate::memory::phys::free_contiguous(physical_start, pages);
    }
}
