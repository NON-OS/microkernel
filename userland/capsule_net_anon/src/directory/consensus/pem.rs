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

//! Reading the base64 object that follows a keyword line.

extern crate alloc;

use alloc::vec::Vec;

use crate::directory::base64::decode;

const BEGIN: &[u8] = b"-----BEGIN ";
const END: &[u8] = b"-----END ";

/// The decoded object that follows the line starting at `at`.
///
pub fn object_after(body: &[u8], at: usize) -> Option<Vec<u8>> {
    let after_line = at + body.get(at..)?.iter().position(|b| *b == b'\n')? + 1;
    let tail = body.get(after_line..)?;
    if !tail.starts_with(BEGIN) {
        return None;
    }
    let first = tail.iter().position(|b| *b == b'\n')? + 1;
    let end = find(tail, END, first)?;
    decode(tail.get(first..end)?)
}

fn find(haystack: &[u8], needle: &[u8], from: usize) -> Option<usize> {
    if haystack.len() < needle.len() {
        return None;
    }
    let last = haystack.len() - needle.len();
    (from..=last).find(|index| &haystack[*index..*index + needle.len()] == needle)
}
