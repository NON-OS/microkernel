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

//! The lexing steps the DMT table parser needs, usable in const.

pub const fn spaces(src: &[u8], mut at: usize) -> usize {
    while at < src.len() && src[at] == b' ' {
        at += 1;
    }
    at
}

/// The number in `radix` (10 or 16) after any spaces, and where it ends.
pub const fn number(src: &[u8], at: usize, radix: u32) -> (u32, usize) {
    let mut at = spaces(src, at);
    let start = at;
    let mut n: u32 = 0;
    while at < src.len() {
        let d = match src[at] {
            c @ b'0'..=b'9' => (c - b'0') as u32,
            c @ b'A'..=b'F' if radix == 16 => (c - b'A') as u32 + 10,
            _ => break,
        };
        n = n * radix + d;
        at += 1;
    }
    assert!(at > start, "number expected");
    (n, at)
}

/// The single letter after any spaces, and where it ends.
pub const fn letter(src: &[u8], at: usize) -> (u8, usize) {
    let at = spaces(src, at);
    assert!(at < src.len(), "letter expected");
    (src[at], at + 1)
}

pub const fn skip_line(src: &[u8], mut at: usize) -> usize {
    while at < src.len() && src[at] != b'\n' {
        at += 1;
    }
    at + 1
}
