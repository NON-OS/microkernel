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

//! Which cache lines a span of memory touches: pure, so the proofs run what
//! the kernel's flush runs.
//!
//! The kernel has no plain invalidate: dropping a line without writing it
//! back would lose whatever else in it was dirty, so a span not starting and
//! ending on line boundaries could take a neighbour's data along. Its flush
//! writes back and drops (clean and invalidate). That is safe because no
//! line holds two owners: DMA grants are whole pages, and a page is a whole
//! number of lines, so every line a grant touches is the grant's alone.

/// The first line and the count of `line`-byte lines covering
/// `[start, start + len)`. None for an empty span, a span that wraps, or a
/// line size that is not a power of two.
pub(crate) const fn lines(start: u64, len: u64, line: u64) -> Option<(u64, u64)> {
    if len == 0 || !line.is_power_of_two() {
        return None;
    }
    let Some(end) = start.checked_add(len) else { return None };
    let Some(up) = end.checked_add(line - 1) else { return None };
    let first = start & !(line - 1);
    Some((first, ((up & !(line - 1)) - first) / line))
}
