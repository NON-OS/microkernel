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

use super::msix_exclusion;
use super::types::{MmioMapError, MmioMapRequest, MmioMapResult};
use super::window::{self, Window, WindowError, PAGE_SIZE};
use crate::hardware::broker::claim;
use crate::hardware::broker::device::BAR_KIND_MMIO;
use crate::hardware::broker::grant::{self, MmioGrant, USER_MMIO_BASE, USER_MMIO_END};
use crate::hardware::broker::table;
use crate::hardware::broker::DeviceRecord;
use crate::memory::addr::PhysAddr;

const FLAGS_KNOWN: u32 = 0;

pub fn map_for_caller(pid: u32, req: MmioMapRequest) -> Result<MmioMapResult, MmioMapError> {
    crate::sys::serial::println(b"[MMIO] claim");
    if req.flags & !FLAGS_KNOWN != 0 {
        return Err(MmioMapError::UnsupportedFlags);
    }
    if req.length == 0 {
        return Err(MmioMapError::ZeroLength);
    }
    let claim = claim::lookup(req.device_id).ok_or(MmioMapError::NotClaimed)?;
    if claim.pid != pid {
        return Err(MmioMapError::NotClaimed);
    }
    if claim.epoch != req.claim_epoch {
        return Err(MmioMapError::StaleEpoch);
    }
    crate::sys::serial::println(b"[MMIO] device");
    let device = table::list()
        .into_iter()
        .find(|r| r.device_id == req.device_id)
        .ok_or(MmioMapError::UnknownDevice)?;
    let bar_idx = req.bar_index as usize;
    if bar_idx >= device.bars.len() || bar_idx >= device.bar_count as usize {
        return Err(MmioMapError::BadBarIndex);
    }
    let bar = device.bars[bar_idx];
    if bar.kind != BAR_KIND_MMIO {
        return Err(MmioMapError::NotMmioBar);
    }
    crate::sys::serial::println(b"[MMIO] msix");
    let protected = msix_exclusion::protected_regions();
    let w = window::window(bar.base, bar.size, req.offset, req.length, &protected)
        .map_err(window_error)?;
    crate::sys::serial::println(b"[MMIO] msix ok");
    note_shared_page(&device, &w);
    let pages = w.page_bytes / PAGE_SIZE;
    let user_va = grant::reserve_user_va(pages).ok_or(MmioMapError::NoVaSpace)?;
    crate::sys::serial::println(b"[MMIO] va");
    let user_va_end =
        user_va.as_u64().checked_add(w.page_bytes).ok_or(MmioMapError::Overflow)?;
    if user_va.as_u64() < USER_MMIO_BASE || user_va_end > USER_MMIO_END {
        return Err(MmioMapError::NoVaSpace);
    }
    crate::sys::serial::println(b"[MMIO] map");
    let phys = PhysAddr::new(w.page_start);
    if crate::memory::paging::map_user_mmio(user_va, phys, w.page_bytes as usize).is_err() {
        return Err(MmioMapError::MapFailed);
    }
    crate::sys::serial::println(b"[MMIO] record");
    let grant_id = grant::allocate_id();
    grant::insert(MmioGrant {
        grant_id,
        pid,
        device_id: req.device_id,
        claim_epoch: claim.epoch,
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

fn window_error(e: WindowError) -> MmioMapError {
    match e {
        WindowError::ZeroLength => MmioMapError::ZeroLength,
        WindowError::Overflow => MmioMapError::Overflow,
        WindowError::BadRange => MmioMapError::BadRange,
        WindowError::Protected => MmioMapError::WouldExposeMsixTable,
    }
}

/// Say on serial when the mapped pages reach past the request into another
/// device's BAR. Allowed (see `window`), but worth one line on a machine
/// whose firmware packed small BARs into one page.
fn note_shared_page(device: &DeviceRecord, w: &Window) {
    let lo = w.page_start;
    let hi = w.page_start + w.page_bytes;
    let shared = table::list().into_iter().filter(|r| r.device_id != device.device_id).any(|r| {
        let count = (r.bar_count as usize).min(r.bars.len());
        r.bars[..count].iter().any(|b| {
            b.kind == BAR_KIND_MMIO
                && b.size != 0
                && b.base < hi
                && b.base.saturating_add(b.size) > lo
        })
    });
    if shared {
        crate::sys::serial::print(b"[MMIO] page shared with another device's BAR at ");
        crate::sys::serial::print_hex(lo);
        crate::sys::serial::println(b"");
    }
}
