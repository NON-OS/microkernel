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

/*
 * The boot memory map's usable regions laid over the span the frame
 * allocator manages. What lies between them is holes, MMIO windows and
 * firmware; what they cover is the machine's RAM. On a q35 guest with
 * 4 GiB the span runs to 6 GiB around a 2 GiB PCI hole, so the span is
 * not the memory.
 */

use super::constants::{align_down, align_up, PAGE_SIZE_U64};

/*
 * Hand `gap` every stretch of `[span_start, span_end)` that no region of
 * `sorted` covers, and return the bytes of the whole frames the regions do
 * cover there. `sorted` is ordered by start; regions that overlap or touch
 * count once. A frame a region covers only in part also touches a gap, and
 * reserving that gap takes it, so it is not counted.
 */
pub(crate) fn walk_usable(
    sorted: &[(u64, u64)],
    span_start: u64,
    span_end: u64,
    mut gap: impl FnMut(u64, u64),
) -> u64 {
    let mut bytes = 0u64;
    let (mut run, mut at) = (span_start, span_start);
    for &(start, end) in sorted {
        if start > at {
            bytes += whole_frames(run, at, span_start, span_end);
            if at < span_end {
                gap(at, start.min(span_end));
            }
            run = start;
        }
        at = at.max(end);
    }
    bytes += whole_frames(run, at, span_start, span_end);
    if at < span_end {
        gap(at, span_end);
    }
    bytes
}

/* Bytes of the whole frames of `[from, to)` inside the span. */
fn whole_frames(from: u64, to: u64, span_start: u64, span_end: u64) -> u64 {
    let first = align_up(from.max(span_start).min(span_end), PAGE_SIZE_U64);
    let last = align_down(to.min(span_end), PAGE_SIZE_U64);
    last.saturating_sub(first)
}
