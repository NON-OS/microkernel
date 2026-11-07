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

/*
 * The writable PT_LOAD ranges of an image, taken once per load. A relative
 * relocation may only write inside one of them; the relocation loop used to
 * re-parse every program header for every entry to ask that, and an image
 * has tens of thousands of entries.
 */

extern crate alloc;

use alloc::vec::Vec;

/// One writable segment, `[start, end)` in link-time addresses.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Range {
    start: u64,
    end: u64,
}

/// The range of a writable PT_LOAD at `vaddr` spanning `memsz` bytes, or
/// `None` if its end overflows, as the per-entry check skipped such a segment.
pub(crate) fn range(vaddr: u64, memsz: u64) -> Option<Range> {
    Some(Range { start: vaddr, end: vaddr.checked_add(memsz)? })
}

/// Whether `len` bytes written at `offset` fall wholly inside one range.
pub(crate) fn covers(ranges: &[Range], offset: u64, len: u64) -> bool {
    let Some(end) = offset.checked_add(len) else {
        return false;
    };
    ranges.iter().any(|r| offset >= r.start && end <= r.end)
}

pub(crate) fn collect(segments: impl Iterator<Item = (u64, u64)>) -> Vec<Range> {
    segments.filter_map(|(vaddr, memsz)| range(vaddr, memsz)).collect()
}
