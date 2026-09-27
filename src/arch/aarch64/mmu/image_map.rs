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

//! The kernel image at page granularity.
//!
//! The boot map describes RAM in 2 MiB blocks, and one block carries one set of
//! permissions. Text and read-only data share the block the image starts in, so
//! mapped as a block the read-only data is executable and the kernel section
//! check reports W^X broken. Each block the image touches is described by a table
//! of 4 KiB pages instead: text is read only and executable, the rest of the
//! image read only and execute never, and the part of the block outside the
//! image keeps the attributes of the region it belongs to.

use super::{state, PageAttributes};

extern "C" {
    static __kernel_image_start: u8;
    static __kernel_text_start: u8;
    static __kernel_text_end: u8;
    static __kernel_rw_start: u8;
}

const PAGE_4K: u64 = 4096;
const BLOCK_2M: u64 = 2 * 1024 * 1024;
const PAGES_PER_BLOCK: usize = 512;

/// Whether the 2 MiB block at `block` holds any part of the text or read-only
/// data. Writable data starts on its own 2 MiB boundary, so it never shares one.
pub(super) fn overlaps_image(block: u64) -> bool {
    let (start, end) = image_bounds();
    block < end && block.saturating_add(BLOCK_2M) > start
}

/// Describe the 2 MiB block at `block` with 4 KiB pages. The block sits at
/// `l2_index` of level 2 table `slot`, under level 1 entry `l1_index`. Returns
/// false having written nothing when no level 3 table exists for that position,
/// and the caller maps the block whole.
///
/// # Safety
///
/// Boot CPU only, before these tables are live, with no other reference to level
/// 2 table `slot` or to the level 3 table at (`l1_index`, `l2_index`).
pub(super) unsafe fn map_image_block(
    slot: usize,
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
    state::l2(slot).set_table(l2_index, state::l3_addr(l1_index, l2_index));
    true
}

fn image_bounds() -> (u64, u64) {
    ((&raw const __kernel_image_start) as u64, (&raw const __kernel_rw_start) as u64)
}

fn text_bounds() -> (u64, u64) {
    ((&raw const __kernel_text_start) as u64, (&raw const __kernel_text_end) as u64)
}
