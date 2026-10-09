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

//! Turning a region into the page-aligned request the broker accepts.
//!
//! Only the part of a region the driver reads is mapped. QEMU sizes the
//! common, ISR and device structures at a page each and the notify area at
//! multiplier times 1024 queues, which with one page per queue is four
//! megabytes of doorbells for a driver that rings two; the caps below keep
//! the request to what can matter. A notify offset past the mapped part is
//! refused by the notify arithmetic, not reached.

use crate::caps::{Region, PAGE_SIZE};

/// The most of a common, ISR or device structure that is mapped: the
/// common configuration is 0x38 bytes and no driver reads past a page of
/// device configuration.
pub const REGION_MAP_MAX: u32 = 0x1000;
/// The most of the notify area that is mapped: four pages, enough for a
/// doorbell per page for the first four queues, or 4096 queues at QEMU's
/// default multiplier of four.
pub const NOTIFY_MAP_MAX: u32 = 0x4000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MapPlan {
    pub bar: u8,
    /// Page-aligned BAR offset and length to ask the broker for.
    pub map_offset: u64,
    pub map_len: u64,
    /// Where the region starts inside that mapping.
    pub in_page: usize,
    /// How many bytes of the region the window covers.
    pub usable: usize,
}

/// The request for the first `cap` bytes of `region`. `None` for an empty
/// region.
pub fn plan(region: Region, cap: u32) -> Option<MapPlan> {
    let usable = core::cmp::min(region.length, cap) as u64;
    if usable == 0 {
        return None;
    }
    let start = region.offset as u64;
    let map_offset = start & !(PAGE_SIZE - 1);
    let in_page = start - map_offset;
    let map_len = (in_page + usable).checked_add(PAGE_SIZE - 1)? & !(PAGE_SIZE - 1);
    Some(MapPlan {
        bar: region.bar,
        map_offset,
        map_len,
        in_page: usize::try_from(in_page).ok()?,
        usable: usize::try_from(usable).ok()?,
    })
}
