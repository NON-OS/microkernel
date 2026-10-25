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
//!      user, read+write, NX, uncached or, asked on a prefetchable BAR for
//!      pages inside it, write-combining once every PAT has it (`cache`);
//!      the kernel's own type where it maps them (`memory_type`)
//!   5. the grant recorded while its claim stands, within the quota (K10)
//!
//! A rejection maps nothing and records nothing. `user_va` is the VA of the
//! first byte asked for and `length` the bytes usable from there; neither
//! need be page aligned (a 2 KiB ABAR at a base ending in 0x800 gets a VA
//! ending in 0x800). An MSI-X table can cut a page-aligned request short.

use super::cache::{self, Cache};
use super::map_bar::{claimed_bar, Claimed};
use super::map_install::install;
use super::map_notes::{cache_error, note_shared_page, window_error};
use super::map_type::memory_type;
use super::memory_type::MMIO_MAP_WC;
use super::types::{MmioMapError, MmioMapRequest, MmioMapResult};
use super::{forbidden_ranges, msix_exclusion, window};

pub fn map_for_caller(pid: u32, req: MmioMapRequest) -> Result<MmioMapResult, MmioMapError> {
    if !cache::known(req.flags) {
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
    let wc_ready = crate::arch::write_combining::write_combining_ready();
    let bar_held = (bar.base, bar.size, bar.flags);
    let cache = cache::cache_for(req.flags, bar_held, (w.page_start, w.page_bytes), wc_ready)
        .map_err(cache_error)?;
    note_shared_page(&device, &w);
    // What cache.rs allows, then the kernel's own type where it maps the pages.
    let wc = cache == Cache::WriteCombined;
    let kind = memory_type(if wc { MMIO_MAP_WC } else { 0 }, &bar, &w)?;
    let user_va = install(pid, &w, kind)?;
    let grant_id = super::map_record::record(pid, &req, epoch, &w, user_va)?;
    let first = user_va.as_u64() + w.in_page;
    Ok(MmioMapResult { user_va: first, length: w.usable, grant_id })
}
