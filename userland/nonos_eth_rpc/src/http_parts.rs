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

//! The pieces of HTTP/1.1 framing an answer is read with.

use alloc::vec::Vec;

pub(crate) fn header<'a>(head: &'a [u8], name: &[u8]) -> Option<&'a [u8]> {
    head.split(|b| *b == b'\n').find_map(|line| {
        let line = line.strip_suffix(b"\r").unwrap_or(line);
        let colon = line.iter().position(|b| *b == b':')?;
        let (key, value) = line.split_at(colon);
        key.eq_ignore_ascii_case(name).then(|| value[1..].trim_ascii())
    })
}

pub(crate) fn decimal(text: &[u8]) -> Option<usize> {
    if text.is_empty() || !text.iter().all(u8::is_ascii_digit) {
        return None;
    }
    text.iter().try_fold(0usize, |n, d| n.checked_mul(10)?.checked_add((d - b'0') as usize))
}

fn hex_size(text: &[u8]) -> Option<usize> {
    let digits = text.split(|b| *b == b';').next()?.trim_ascii();
    if digits.is_empty() {
        return None;
    }
    digits.iter().try_fold(0usize, |n, d| {
        let v = (*d as char).to_digit(16)? as usize;
        n.checked_mul(16)?.checked_add(v)
    })
}

/* The chunks of a chunked body joined, or None until the last chunk. */
pub(crate) fn dechunk(mut rest: &[u8]) -> Option<Vec<u8>> {
    let mut out = Vec::new();
    loop {
        let eol = rest.windows(2).position(|w| w == b"\r\n")?;
        let size = hex_size(&rest[..eol])?;
        rest = &rest[eol + 2..];
        if size == 0 {
            return Some(out);
        }
        let end = size.checked_add(2)?;
        if rest.len() < end || &rest[size..end] != b"\r\n" {
            return None;
        }
        out.extend_from_slice(&rest[..size]);
        rest = &rest[end..];
    }
}
