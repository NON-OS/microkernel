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

//! Reading the few JSON shapes a log answer has, as bytes: the objects of an
//! array and a string field of an object. Strings are skipped while
//! braces are counted, so a brace inside a value never moves a boundary.

use alloc::vec::Vec;

/// The top-level `{...}` objects in `arr`, in order.
pub fn objects(arr: &[u8]) -> Vec<&[u8]> {
    let (mut out, mut depth, mut start) = (Vec::new(), 0u32, 0usize);
    let (mut in_string, mut escaped) = (false, false);
    for (i, &b) in arr.iter().enumerate() {
        if in_string {
            match (escaped, b) {
                (true, _) => escaped = false,
                (false, b'\\') => escaped = true,
                (false, b'"') => in_string = false,
                _ => {}
            }
            continue;
        }
        match b {
            b'"' => in_string = true,
            b'{' if depth == 0 => (start, depth) = (i, 1),
            b'{' => depth += 1,
            b'}' if depth == 1 => {
                depth = 0;
                out.push(&arr[start..=i]);
            }
            b'}' => depth = depth.saturating_sub(1),
            _ => {}
        }
    }
    out
}

/// The quoted value of `"key":"..."` in `obj`.
pub fn string_field<'a>(obj: &'a [u8], key: &[u8]) -> Option<&'a [u8]> {
    let mut pat = Vec::with_capacity(key.len() + 4);
    pat.push(b'"');
    pat.extend_from_slice(key);
    pat.extend_from_slice(b"\":\"");
    let start = obj.windows(pat.len()).position(|w| w == pat)? + pat.len();
    let len = obj[start..].iter().position(|b| *b == b'"')?;
    Some(&obj[start..start + len])
}
