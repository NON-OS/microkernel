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

//! The kernel image at page granularity. A 2 MiB block carries one set of
//! permissions, and text and read-only data share the block the image starts
//! in, so mapped as a block the read-only data is executable and the kernel
//! section check reports W^X broken. Each block the image touches is a table of
//! 4 KiB pages instead: text read only and executable, the rest of the image
//! read only and execute never, the rest of the block as its region.

use super::geometry::{BLOCK_2M, ENTRIES as PAGES_PER_BLOCK};
use super::image_bounds::{image_bounds, text_bounds};
use super::state::{self, PageTable};
use super::PageAttributes;

const PAGE_4K: u64 = 4096;

/// Whether the 2 MiB block at `block` holds any part of the text or read-only
/// data. Writable data starts on its own 2 MiB boundary, so it never shares one.
pub(super) fn overlaps_image(block: u64) -> bool {
    let (start, end) = image_bounds();
    block < end && block.saturating_add(BLOCK_2M) > start
}

/// Describe the 2 MiB block at `block`, entry `l2_index` of `l2` under level 1
/// entry `l1_index`, with 4 KiB pages. False, having written nothing, when no
/// level 3 table exists for that position: the caller maps the block whole.
///
/// # Safety
/// Boot CPU, before the tables are live; nothing else refers to that level 3.
pub(super) unsafe fn map_image_block(
    l2: &mut PageTable,
    l1_index: usize,
    l2_index: usize,
    block: u64,
    outside: &PageAttributes,
) -> bool {
    if l1_index >= state::L3_L1_SPAN || l2_index >= PAGES_PER_BLOCK {
        return false;
    }
    let code = PageAttributes::kernel_code();
    let rodata = PageAttributes::kernel_rodata();
    let (image_start, image_end) = image_bounds();
    let (text_start, text_end) = text_bounds();
    let table = state::l3(l1_index, l2_index);
    let mut page = block;
    for index in 0..PAGES_PER_BLOCK {
        let next = page.saturating_add(PAGE_4K);
        let attrs = if page < text_end && next > text_start {
            &code
        } else if page < image_end && next > image_start {
            &rodata
        } else {
            outside
        };
        table.set_page(index, page, attrs);
        page = next;
    }
    l2.set_table(l2_index, state::l3_addr(l1_index, l2_index));
    true
}
