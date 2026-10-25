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

//! Where the shown tail of a typed field starts, pure for the wallet proofs.

/* Where the shown tail of a field's text starts: the first character from
 * which the rest fits in `room`. `width` is the measure of a tail and
 * shrinks as the tail does, so a halving search asks it about a dozen
 * times for a 2,009-letter address, not once per letter dropped. */
pub fn tail_start(text: &str, room: i32, width: impl Fn(&str) -> i32) -> usize {
    if width(text) <= room {
        return 0;
    }
    let starts: alloc::vec::Vec<usize> = text.char_indices().map(|(i, _)| i).collect();
    let (mut lo, mut hi) = (0usize, starts.len());
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        let at = starts.get(mid).copied().unwrap_or(text.len());
        if width(text.get(at..).unwrap_or("")) <= room {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    starts.get(lo).copied().unwrap_or(text.len())
}
