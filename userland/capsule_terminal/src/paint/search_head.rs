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

//! What the prompt says while a history search runs.
//!
//! It used to be a lone `?` and the needle cut to the width of the working
//! directory, so `he` typed at `~` showed as `?h`. Now it names the mode, says
//! when nothing matches, and shows the needle in full, or its end when the
//! room runs out, since the end is where the reader is typing.

/// The label while the needle has a match.
pub const FOUND: &[u8] = b"history ";
/// The label while it has none.
pub const MISSING: &[u8] = b"no match ";
/// What sits between the needle and the match: two cells.
pub const GAP: usize = 2;

/// The label, and the part of the needle that fits in `room` cells after it
/// and the gap. The part is the needle's end, cut at a character boundary.
pub fn head(needle: &[u8], found: bool, room: usize) -> (&'static [u8], &[u8]) {
    let label = if found { FOUND } else { MISSING };
    let left = room.saturating_sub(label.len() + GAP);
    let mut start = 0;
    while cells(&needle[start..]) > left {
        start += 1;
        while start < needle.len() && needle[start] & 0xC0 == 0x80 {
            start += 1;
        }
    }
    (label, &needle[start..])
}

/// How many cells the prompt takes for this head.
pub fn width(label: &[u8], shown: &[u8]) -> usize {
    label.len() + cells(shown) + GAP
}

/// One cell for every character: count the bytes that start one.
pub fn cells(s: &[u8]) -> usize {
    s.iter().filter(|b| **b & 0xC0 != 0x80).count()
}
