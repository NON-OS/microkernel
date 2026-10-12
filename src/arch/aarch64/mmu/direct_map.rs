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

//! The kernel's direct map: every physical page of RAM reachable at its
//! address plus `KERNEL_SPACE_START`.
//!
//! Shared code turns a physical address into a pointer by adding that constant,
//! which is how the paging manager reads a page table it has only the physical
//! address of. Without this the first such read faults on an address nothing
//! describes, and the identity map alone cannot serve it: the two windows differ
//! by more than the top bits, so they need separate tables.

use core::sync::atomic::{AtomicU64, Ordering};

use super::geometry::{BLOCK_2M, ENTRIES, GIB};
use super::l2_pool::l2_under;
use super::{state, PageAttributes};

/// The first physical address the direct map could not reach, or `u64::MAX`.
static DIRECT_END: AtomicU64 = AtomicU64::new(u64::MAX);

/// Where the direct map stops covering RAM. The physical allocator is held
/// below it: a frame it cannot reach would fault on first use.
pub fn direct_map_end() -> u64 {
    DIRECT_END.load(Ordering::Relaxed)
}

/// Map `[base, base + size)` into the direct window with 2 MiB blocks.
/// `KERNEL_SPACE_START` is 512 GiB aligned, so the level 1 index is the
/// physical gigabyte.
///
/// # Safety
///
/// Boot CPU, before these tables are live.
pub(super) unsafe fn map_direct_range(base: u64, size: u64) {
    // Writable and execute-never: this window exists to read and write memory
    // the kernel knows only by physical address, never to run from.
    let attrs = PageAttributes::kernel_data();
    let mut phys = base & !(BLOCK_2M - 1);
    let end = base.saturating_add(size);
    while phys < end {
        let l1_idx = (phys / GIB) as usize;
        let l2 = if l1_idx < ENTRIES { l2_under(state::l1_high(), l1_idx) } else { None };
        let Some(l2) = l2 else {
            DIRECT_END.fetch_min(phys, Ordering::Relaxed);
            return;
        };
        l2.set_block(((phys / BLOCK_2M) % ENTRIES as u64) as usize, phys, &attrs);
        phys = phys.saturating_add(BLOCK_2M);
    }
}
