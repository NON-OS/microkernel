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

//! The single in-kernel path that turns a PCI BAR slice into a
//! mapping in a capsule address space. Five steps in order:
//!
//!   1. resolve the caller's claim and verify its epoch is fresh
//!   2. resolve the device record and the requested BAR
//!   3. validate BAR containment of the request and find the pages it
//!      touches, kept off every MSI-X table and PBA (`window`)
//!   4. reserve a user VA window in the per-capsule MMIO region
//!      and install pages with user / read+write / uncached / NX
//!   5. record the grant so revocation can find and undo it
//!
//! On any rejection no mapping is installed and no record is made.
//!
//! The result's `user_va` is the VA of the requested first byte, and
//! `length` the bytes usable from there. Neither base nor length need be
//! page aligned: a request inside a page (a 2 KiB AHCI ABAR at a base
//! ending in 0x800) maps the whole page and returns a VA ending in the same
//! 0x800. A page-aligned request gets a page-aligned VA and its own length,
//! or less when an MSI-X table cuts it short, as before.

use super::map_bar::{claimed_bar, Claimed};
use super::map_install::install;
use super::map_notes::{note_shared_page, window_error};
use super::msix_exclusion;
use super::types::{MmioMapError, MmioMapRequest, MmioMapResult};
use super::window;
use crate::hardware::broker::grant::{self, MmioGrant};

const FLAGS_KNOWN: u32 = 0;

pub fn map_for_caller(pid: u32, req: MmioMapRequest) -> Result<MmioMapResult, MmioMapError> {
    if req.flags & !FLAGS_KNOWN != 0 {
        return Err(MmioMapError::UnsupportedFlags);
    }
    if req.length == 0 {
        return Err(MmioMapError::ZeroLength);
    }
    let Claimed { epoch, device, bar } = claimed_bar(pid, &req)?;
    let protected = msix_exclusion::protected_regions();
    let w = window::window(bar.base, bar.size, req.offset, req.length, &protected)
        .map_err(window_error)?;
    note_shared_page(&device, &w);
    let user_va = install(&w)?;
    let grant_id = grant::allocate_id();
    grant::insert(MmioGrant {
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
    });
    let first = user_va.as_u64() + w.in_page;
    Ok(MmioMapResult { user_va: first, length: w.usable, grant_id })
}
