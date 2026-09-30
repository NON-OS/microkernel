// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Taking frames out of the allocator's hands for good: the holes between
//! the firmware's usable regions, and pools other allocators own.

use super::super::bitmap;
use super::super::constants::PAGE_SIZE_U64;
use super::super::types::AllocatorState;

/// Mark every frame that overlaps `[start, end)` and lies in the managed
/// span as in use. Frames outside the span are not the allocator's anyway.
pub fn reserve_range(state: &mut AllocatorState, start: u64, end: u64) {
    if !state.is_initialized() || end <= start {
        return;
    }
    let span_end = state.frame_start + state.frame_count as u64 * PAGE_SIZE_U64;
    let first = start.max(state.frame_start) / PAGE_SIZE_U64 * PAGE_SIZE_U64;
    let last = end.min(span_end);
    if last <= first {
        return;
    }
    let from = ((first - state.frame_start) / PAGE_SIZE_U64) as usize;
    let count = (last - first).div_ceil(PAGE_SIZE_U64) as usize;
    /*
     * SAFETY: eK@nonos.systems - `from + count` frames lie inside the managed
     * span, so every bit is inside the bitmap sized for that span.
     */
    unsafe {
        let _ = bitmap::set_bit_range(state.bitmap_ptr, from, count.min(state.frame_count - from));
    }
}

/// Frames in `[start, end)` are never handed out.
pub fn phys_reserve(start: u64, end: u64) {
    reserve_range(&mut super::api::ALLOCATOR.lock(), start, end)
}
