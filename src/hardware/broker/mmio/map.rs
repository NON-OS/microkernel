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
//! The single in-kernel path that turns a PCI BAR slice into a mapping in a
//! capsule address space, in order:
//!
//!   1. the caller's claim, its epoch fresh, held until step 5 (map_bar.rs)
//!   2. the device record and the BAR it names
//!   3. the pages the request touches, kept off every MSI-X table and PBA
//!      (`window`) and refused whole over RAM, the kernel image or an
//!      IOMMU's registers (`forbidden`, K6)
//!   4. a user VA from the holder's own MMIO window, the pages installed
//!      user, read+write, NX, uncached or, on a prefetchable BAR when asked,
//!      write-combining (`memory_type`)
//!   5. the grant recorded while its claim stands, within the quota (K10)
//!
//! A rejection maps nothing and records nothing. `user_va` is the VA of the
//! first byte asked for and `length` the bytes usable from there; neither
//! need be page aligned (a 2 KiB ABAR at a base ending in 0x800 gets a VA
//! ending in 0x800). An MSI-X table can cut a page-aligned request short.

use super::map_bar::{claimed_bar, Claimed};
use super::map_install::install;
use super::map_notes::{note_shared_page, window_error};
use super::map_type::memory_type;
use super::memory_type::MMIO_MAP_WC;
use super::types::{MmioMapError, MmioMapRequest, MmioMapResult};
use super::{forbidden_ranges, msix_exclusion, window};
use crate::hardware::broker::grant::{self, MmioGrant};

pub fn map_for_caller(pid: u32, req: MmioMapRequest) -> Result<MmioMapResult, MmioMapError> {
    if req.flags & !MMIO_MAP_WC != 0 {
        return Err(MmioMapError::UnsupportedFlags);
    }
    if req.length == 0 {
        return Err(MmioMapError::ZeroLength);
    }
    let Claimed { epoch, device, bar, held: _held } = claimed_bar(pid, &req)?;
    let protected = msix_exclusion::protected_regions();
    let w = window::window(bar.base, bar.size, req.offset, req.length, &protected)
        .map_err(window_error)?;
    forbidden_ranges::admit(w.page_start, w.page_bytes)?;
    note_shared_page(&device, &w);
    let kind = memory_type(req.flags, &bar, &w)?;
    let user_va = install(pid, &w, kind)?;
    let grant_id = grant::allocate_id();
    super::map_record::record(MmioGrant {
        grant_id,
        pid,
        device_id: req.device_id,
        claim_epoch: epoch,
        bar_index: req.bar_index,
        // The record holds the whole pages, which is what unmap gives back.
        physical_start: w.page_start,
        user_va: user_va.as_u64(),
        length: w.page_bytes,
        flags: req.flags,
    })?;
    let first = user_va.as_u64() + w.in_page;
    Ok(MmioMapResult { user_va: first, length: w.usable, grant_id })
}
