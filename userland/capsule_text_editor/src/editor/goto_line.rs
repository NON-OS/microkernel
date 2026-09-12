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

//! Jumping to a line by number.
//!
//! The one navigation everybody has in their fingers and this editor had no
//! way to do. A compiler names a line, a colleague names a line, and without
//! this the only way there is to scroll and count.
//!
//! Kept free of any state type so the arithmetic can be checked on a host: the
//! whole risk here is off-by-one, and an editor that lands one line away from
//! where it was told is worse than one that cannot jump at all.

/// Byte offset where the one-based line `n` starts.
///
/// Clamped rather than refused. Asking for line 900 of a 400-line file means
/// the end, which is where a reader who typed it is trying to get, and
/// refusing would make them ask twice.
///
/// Line 0 is treated as line 1: the numbering people type is one-based, and
/// nothing sensible is at "line zero".
pub fn offset_of_line(buf: &[u8], n: usize) -> usize {
    if n <= 1 {
        return 0;
    }
    let mut seen = 1usize;
    for (i, &b) in buf.iter().enumerate() {
        if b == b'\n' {
            seen += 1;
            if seen == n {
                return i + 1;
            }
        }
    }
    // Past the end: the start of the final line, not the very end, so the
    // caret lands somewhere a person can read rather than after the last byte.
    buf.iter().rposition(|&b| b == b'\n').map(|p| p + 1).unwrap_or(0)
}

/// Read a line number out of the prompt.
///
/// Only digits. A prompt that accepted `12abc` and jumped to 12 would be
/// guessing at what was meant, and this is a navigation nobody checks after
/// pressing Enter.
pub fn parse_line_number(text: &[u8]) -> Option<usize> {
    if text.is_empty() || !text.iter().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let mut n = 0usize;
    for b in text {
        // Saturating: a number too large to hold is still a number, and it
        // means the end of the file, which is what the clamp above does.
        n = n.saturating_mul(10).saturating_add((b - b'0') as usize);
    }
    Some(n)
}
