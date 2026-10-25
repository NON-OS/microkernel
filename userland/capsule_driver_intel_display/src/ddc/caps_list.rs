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

//! Finding the vcp list in a capabilities string: the key at depth one,
//! and its value up to the parenthesis that closes it.

/// The inside of the top-level `vcp(...)`, or None when there is none or
/// it never closes.
pub fn vcp_list(s: &[u8]) -> Option<&[u8]> {
    let mut depth = 0usize;
    for (i, &b) in s.iter().enumerate() {
        match b {
            b'(' => depth += 1,
            b')' => depth = depth.checked_sub(1)?,
            _ if depth == 1 && key_at(s, i) => return inside(&s[i + 4..]),
            _ => {}
        }
    }
    None
}

/// Whether `vcp(` starts at `i`, as a key and not the end of another word.
fn key_at(s: &[u8], i: usize) -> bool {
    let word = s.get(i..i + 4).is_some_and(|w| w.eq_ignore_ascii_case(b"vcp("));
    word && (i == 0 || !s[i - 1].is_ascii_alphanumeric())
}

/// What lies between `s`'s start and the parenthesis that closes it.
fn inside(s: &[u8]) -> Option<&[u8]> {
    let mut depth = 1usize;
    for (i, &b) in s.iter().enumerate() {
        depth = match b {
            b'(' => depth + 1,
            b')' => depth - 1,
            _ => depth,
        };
        if depth == 0 {
            return Some(&s[..i]);
        }
    }
    None
}
