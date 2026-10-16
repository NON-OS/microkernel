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

//! One 2 MiB block of RAM turned Normal Non-cacheable for the kernel, for
//! buffers a device that does not snoop shares with a capsule: the capsule
//! maps them Non-cacheable too, so no side holds a cacheable alias another
//! side's writes can go stale behind (Arm ARM, DDI 0487, B2: mismatched
//! memory attributes). The direct map's block changes type; the identity
//! map's block, which nothing reaches these frames through, goes.
//!
//! Each change is break-before-make: the old entry made invalid and every
//! CPU's TLB told before the new one is written. Nothing else lives in the
//! block, which the boot path cut from the reserved DMA pool, so nothing
//! misses it while it is invalid.

use super::geometry::{BLOCK_2M, ENTRIES, GIB, KERNEL_SPACE_START};
use super::l2_pool::existing_l2;
use super::{image_map, state, tlb, MemoryType, PageAttributes};

/// Kernel data, read and write, never executed, Normal Non-cacheable.
const UNCACHED: PageAttributes =
    PageAttributes::new(MemoryType::NormalNC, true, false, false, true, true, true);

pub fn make_block_uncached(pa: u64) -> Result<(), &'static str> {
    if pa % BLOCK_2M != 0 || image_map::overlaps_image(pa) || pa / GIB >= ENTRIES as u64 {
        return Err("not an aligned block of RAM outside the image");
    }
    let (l1, l2) = ((pa / GIB) as usize, ((pa / BLOCK_2M) % ENTRIES as u64) as usize);
    let direct = KERNEL_SPACE_START + pa;
    // SAFETY: the boot CPU, before any capsule exists; one borrow each.
    let high = unsafe { existing_l2(state::l1_high(), l1) }.ok_or("no direct map there")?;
    if high.block_address(l2) != Some(pa) {
        return Err("the direct map does not hold that block whole");
    }
    // Written out while still cacheable, so no dirty line survives the change.
    crate::arch::cache_flush::flush_and_fence(direct, BLOCK_2M);
    high.clear_entry(l2);
    tlb::flush_tlb_page(direct);
    high.set_block(l2, pa, &UNCACHED);
    tlb::flush_tlb_page(direct);
    // SAFETY: as above.
    if let Some(low) = unsafe { existing_l2(state::l1(), l1) } {
        if low.block_address(l2) == Some(pa) {
            low.clear_entry(l2);
            tlb::flush_tlb_page(pa);
        }
    }
    // A line filled speculatively between the clean and the break is clean,
    // and dropped here, through the mapping that is left.
    crate::arch::cache_flush::flush_and_fence(direct, BLOCK_2M);
    Ok(())
}
