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

//! The exact byte range a consensus signature covers.

const START: &[u8] = b"network-status-version";
const END: &[u8] = b"\ndirectory-signature";

/*
 * router_get_hash_impl_helper in sigcommon.c, called with this start, this end
 * and a terminator of ' '. So the range runs to the first space after the first
 * "\ndirectory-signature", trailing space included.
 *
 * An authority certificate uses the same helper with a terminator of '\n'
 * instead. Off by one either way does not produce a wrong answer for some
 * documents, it produces a digest that matches for none of them.
 */
pub fn signed_range(body: &[u8]) -> Option<(usize, usize)> {
    let start = find(body, START, 0)?;
    if start != 0 && body.get(start.wrapping_sub(1)) != Some(&b'\n') {
        return None;
    }
    let marker = find(body, END, start + START.len())?;
    let after = marker + END.len();
    let space = body.get(after..)?.iter().position(|b| *b == b' ')?;
    Some((start, after + space + 1))
}

fn find(haystack: &[u8], needle: &[u8], from: usize) -> Option<usize> {
    if needle.is_empty() || haystack.len() < needle.len() {
        return None;
    }
    let last = haystack.len() - needle.len();
    (from..=last).find(|at| &haystack[*at..*at + needle.len()] == needle)
}
