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

use super::super::flags::DMA_MAP_DMA32;
use super::super::types::{DmaMapError, DmaMapRequest, DmaMapResult};
use super::fail::fail;
use super::placed::Placed;
use super::{alloc, bus_window, given, install, record, uncached, validate};
use crate::hardware::broker::reserved;

// `MkDmaMap`: validate -> alloc+zero frames -> install user pages ->
// record. Each step is a single responsibility in its own file; this
// function is the transaction boundary and owns the rollback chain.
pub fn map_for_caller(pid: u32, req: DmaMapRequest) -> Result<DmaMapResult, DmaMapError> {
    let claim_epoch = match validate::validate(&req, pid) {
        Ok(epoch) => epoch,
        Err(e) => return fail("validate", e),
    };
    let pages = req.length / validate::PAGE_SIZE;
    let alloc_flags = match bus_window::alloc_flags(req.device_id, req.flags) {
        Ok(flags) => flags,
        Err(e) => return fail("window", e),
    };

    let uncached = uncached::needs_uncached_frames(req.device_id, req.flags);
    let phys_start = match alloc::alloc_and_zero(pages, req.length, alloc_flags, uncached) {
        Ok(start) => start,
        Err(e) => return fail("alloc", e),
    };
    // RAM the allocator holds is never firmware's; if the memory map and
    // the namespace disagree, the device gets neither.
    if reserved::refuses(b"DMA", reserved::MEMORY, phys_start, req.length, false) {
        alloc::free(phys_start, pages);
        return fail("reserved", DmaMapError::NoMemory);
    }

    let user_va = match install::install(pid, pages, req.length, phys_start, req.flags) {
        Ok(va) => va,
        Err(e) => {
            alloc::free(phys_start, pages);
            return fail("install", e);
        }
    };
    let placed = Placed { pid, phys_start, pages, length: req.length, user_va };

    let (device_addr, confined) = match given::device_address(pid, &req, &placed) {
        Ok(got) => got,
        Err((step, e)) => return fail(step, e),
    };
    // A 32-bit device is handed an address that fits its descriptor or nothing.
    // Frames and IOVAs are both drawn below 4GB, so this holds by construction;
    // it is checked so a change to either cannot truncate an address in silence.
    if req.flags & DMA_MAP_DMA32 != 0 && !alloc::fits_below_4g(device_addr, pages) {
        placed.undo_mapped(pid, device_addr, confined);
        return fail("dma32", DmaMapError::Above4G);
    }

    // Entered only while the claim holds and within the quota (K10); a
    // release since validate finds nothing to revoke, so the map comes out.
    let grant_id = match record::record(pid, &req, claim_epoch, &placed, device_addr, confined) {
        Ok(id) => id,
        Err(e) => {
            placed.undo_mapped(pid, device_addr, confined);
            return fail("record", e);
        }
    };
    Ok(DmaMapResult { user_va, device_addr, length: req.length, grant_id })
}
