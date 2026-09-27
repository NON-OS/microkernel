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

use super::super::records;
use super::super::types::{DmaGrant, DmaMapError, DmaMapRequest, DmaMapResult};
use super::fail::fail;
use super::{alloc, install, validate};

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

    let user_va = match install::install(pages, req.length, phys_start) {
        Ok(va) => va,
        Err(e) => {
            alloc::free(phys_start, pages);
            return fail("install", e);
        }
    };

    let Some((device_addr, confined)) =
        crate::hardware::broker::confine::map(pid, req.device_id, phys_start, req.length)
    else {
        install::uninstall(user_va, req.length);
        alloc::free(phys_start, pages);
        return fail("confine", DmaMapError::MapFailed);
    };

    let grant_id = records::allocate_id();
    records::insert(DmaGrant {
        grant_id,
        pid,
        device_id: req.device_id,
        claim_epoch,
        physical_start: phys_start,
        user_va,
        length: req.length,
        flags: req.flags,
        device_addr,
        confined,
    });

    Ok(DmaMapResult { user_va, device_addr, length: req.length, grant_id })
}
