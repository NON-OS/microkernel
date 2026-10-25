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

//! What an MMIO map reports beside the grant: a window refusal as the map's
//! error, and a page it shares with another device's BAR, which the map
//! allows and names.

use super::cache::{Refused, PREFETCH};
use super::types::MmioMapError;
use super::window::{Window, WindowError};
use crate::hardware::broker::device::{BAR_FLAG_PREFETCH, BAR_KIND_MMIO};
use crate::hardware::broker::table;
use crate::hardware::broker::DeviceRecord;

pub(super) fn window_error(e: WindowError) -> MmioMapError {
    match e {
        WindowError::ZeroLength => MmioMapError::ZeroLength,
        WindowError::Overflow => MmioMapError::Overflow,
        WindowError::BadRange => MmioMapError::BadRange,
        WindowError::Protected => MmioMapError::WouldExposeMsixTable,
    }
}

// cache.rs reads the BAR record's prefetchable bit by its own name.
const _: () = assert!(PREFETCH == BAR_FLAG_PREFETCH);

pub(super) fn cache_error(e: Refused) -> MmioMapError {
    match e {
        Refused::Unknown => MmioMapError::UnsupportedFlags,
        Refused::NotPrefetchable => MmioMapError::WcNotPrefetchable,
        Refused::PastBar => MmioMapError::WcPastBar,
    }
}

/// Say on serial when the mapped pages reach past the request into another
/// device's BAR. Allowed (see `window`), but worth one line on a machine
/// whose firmware packed small BARs into one page.
pub(super) fn note_shared_page(device: &DeviceRecord, w: &Window) {
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
    // No physical address on the line (K10).
    if shared {
        crate::sys::serial::println(b"[MMIO] page shared with another device's BAR");
    }
}
