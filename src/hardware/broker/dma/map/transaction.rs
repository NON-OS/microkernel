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
use super::{alloc, install, record, validate};

// `MkDmaMap`: validate -> alloc+zero frames -> install user pages ->
// record. Each step is a single responsibility in its own file; this
// function is the transaction boundary and owns the rollback chain.
pub fn map_for_caller(pid: u32, req: DmaMapRequest) -> Result<DmaMapResult, DmaMapError> {
    let claim_epoch = match validate::validate(&req, pid) {
        Ok(epoch) => epoch,
        Err(e) => return fail("validate", e),
    };
    let pages = req.length / validate::PAGE_SIZE;

    let phys_start = match alloc::alloc_and_zero(pages, req.length, req.flags) {
        Ok(start) => start,
        Err(e) => return fail("alloc", e),
    };

    let user_va = match install::install(pages, req.length, phys_start, req.flags) {
        Ok(va) => va,
        Err(e) => {
            alloc::free(phys_start, pages);
            return fail("install", e);
        }
    };
    let placed = Placed { phys_start, pages, length: req.length, user_va };

    let Some((device_addr, confined)) =
        crate::hardware::broker::confine::map(pid, req.device_id, phys_start, req.length)
    else {
        placed.undo();
        return fail("confine", DmaMapError::MapFailed);
    };
    /*
     * A 32-bit device is handed an address that fits its descriptor or nothing.
     * The frames were placed below 4GB and an IOVA is drawn below 4GB, so this
     * holds by construction; it is checked so a change to either cannot hand the
     * device a truncated address in silence.
     */
    if req.flags & DMA_MAP_DMA32 != 0 && !alloc::fits_below_4g(device_addr, pages) {
        placed.undo_mapped(pid, device_addr, confined);
        return fail("dma32", DmaMapError::Above4G);
    }

    let grant_id = record::record(pid, &req, claim_epoch, &placed, device_addr, confined);
    Ok(DmaMapResult { user_va, device_addr, length: req.length, grant_id })
}
