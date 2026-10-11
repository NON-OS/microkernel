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
 * The register walks the caller's pages before it takes the pin gate. A
 * sibling thread's munmap of the same range in that window freed the frames
 * the walk had just read, and the surface kept them: the next owner of each
 * frame shared it with whoever attached the surface. Under the gate the
 * register refuses a range an mmap or munmap is changing (`busy`), and walks
 * again to see that every page still maps the frame it read.
 */

/// True when `[a, a_end)` and `[b, b_end)` share a byte.
pub fn ranges_meet(a: u64, a_end: u64, b: u64, b_end: u64) -> bool {
    a < b_end && b < a_end
}

/// True when the second walk (`now`, one entry per page, None for a page no
/// longer mapped) finds every frame of the first (`walked`) where it was.
pub fn still_mapped(walked: &[u64], now: impl IntoIterator<Item = Option<u64>>) -> bool {
    let mut seen = 0;
    for (i, frame) in now.into_iter().enumerate() {
        if frame.is_none() || walked.get(i) != frame.as_ref() {
            return false;
        }
        seen += 1;
    }
    seen == walked.len()
}
