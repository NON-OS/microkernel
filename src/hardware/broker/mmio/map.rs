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

//! The one path from a PCI BAR slice to a mapping in a capsule: the claim
//! and its epoch, the BAR (map_resolve.rs); the pages the request touches,
//! kept off every MSI-X table and PBA (window.rs) and off RAM, the kernel
//! image and the IOMMUs' registers (forbidden.rs); a user VA in the MMIO
//! region, mapped user, read+write, uncached, NX; then the grant's record,
//! while the claim still holds (map_record.rs). A rejection maps nothing. `user_va` is the VA of the first byte asked for
//! (a 2 KiB ABAR at ...800 gets a VA ending 0x800), `length` the bytes usable.

use super::map_shared::{note_shared_page, window_error};
use super::types::{MmioMapError, MmioMapRequest, MmioMapResult};
use super::window::{self, PAGE_SIZE};
use super::{forbidden_ranges, msix_exclusion};
use crate::hardware::broker::grant::{self, USER_MMIO_BASE, USER_MMIO_END};
use crate::memory::addr::PhysAddr;

const FLAGS_KNOWN: u32 = 0;

pub fn map_for_caller(pid: u32, req: MmioMapRequest) -> Result<MmioMapResult, MmioMapError> {
    if req.flags & !FLAGS_KNOWN != 0 {
        return Err(MmioMapError::UnsupportedFlags);
    }
    if req.length == 0 {
        return Err(MmioMapError::ZeroLength);
    }
    let (claim, device, bar) = super::map_resolve::resolve(pid, &req)?;
    let protected = msix_exclusion::protected_regions();
    let w = window::window(bar.base, bar.size, req.offset, req.length, &protected)
        .map_err(window_error)?;
    forbidden_ranges::admit(w.page_start, w.page_bytes)?;
    note_shared_page(&device, &w);
    let pages = w.page_bytes / PAGE_SIZE;
    let user_va = grant::reserve_user_va(pages).ok_or(MmioMapError::NoVaSpace)?;
    let user_va_end = user_va.as_u64().checked_add(w.page_bytes).ok_or(MmioMapError::Overflow)?;
    if user_va.as_u64() < USER_MMIO_BASE || user_va_end > USER_MMIO_END {
        return Err(MmioMapError::NoVaSpace);
    }
    let phys = PhysAddr::new(w.page_start);
    if crate::memory::paging::map_user_mmio(user_va, phys, w.page_bytes as usize).is_err() {
        return Err(MmioMapError::MapFailed);
    }
    let grant_id = super::map_record::record(pid, &req, claim.epoch, &w, user_va)?;
    let first = user_va.as_u64() + w.in_page;
    Ok(MmioMapResult { user_va: first, length: w.usable, grant_id })
}
