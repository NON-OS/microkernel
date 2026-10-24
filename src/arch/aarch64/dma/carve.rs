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

//! Where the non-cacheable block comes from: one 2 MiB block, aligned, cut
//! from the low DMA pool, so it lies wherever the pool's devices can reach.
//! The rest of the pool stays one run, the larger of the two the cut can
//! leave, so the pool's own allocator keeps working on a single range.

/// A level 2 block: what the direct map maps in one descriptor, and so the
/// least that can change memory type without touching anything else.
pub const BLOCK: u64 = 2 << 20;

/// `(block, rest_base, rest_bytes)` for a pool of `bytes` at `base`, or
/// None when no aligned block fits inside it.
pub const fn split(base: u64, bytes: u64) -> Option<(u64, u64, u64)> {
    let Some(end) = base.checked_add(bytes) else { return None };
    let Some(up) = base.checked_add(BLOCK - 1) else { return None };
    let head = up & !(BLOCK - 1);
    let tail = (end & !(BLOCK - 1)).wrapping_sub(BLOCK);
    if head.checked_add(BLOCK).is_none() || head + BLOCK > end {
        return None;
    }
    // The block at the first aligned address leaves what follows it; the
    // block at the last leaves what precedes it. Keep the larger run.
    let after_head = end - (head + BLOCK);
    let before_tail = tail - base;
    if before_tail >= after_head {
        Some((tail, base, before_tail))
    } else {
        Some((head, head + BLOCK, after_head))
    }
}
