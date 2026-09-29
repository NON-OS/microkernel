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

//! An HTTP/1.1 reply's framing: where the header ends, how long the body
//! says it is, and whether all of it has arrived. A socket read that returns
//! nothing means "not yet", not "done", so the length is what ends a read.

use alloc::vec::Vec;

/// The header's end and the body length it states, once the header is in.
pub fn framing(reply: &[u8]) -> Option<(usize, Option<usize>)> {
    let end = reply.windows(4).position(|w| w == b"\r\n\r\n")? + 4;
    let length = reply[..end].split(|b| *b == b'\n').find_map(|line| {
        let (name, value) = line.split_at(line.iter().position(|b| *b == b':')?);
        let value = core::str::from_utf8(&value[1..]).ok()?.trim();
        name.eq_ignore_ascii_case(b"content-length").then(|| value.parse().ok())?
    });
    Some((end, length))
}

/// Every byte the header promised has arrived.
pub fn complete(reply: &[u8]) -> bool {
    match framing(reply) {
        Some((end, Some(len))) => end.checked_add(len).is_some_and(|want| reply.len() >= want),
        _ => false,
    }
}

/// The body of a 200 reply, cut to its stated length, in place. A reply that
/// is not a 200, or whose body stops short of its length, is nothing: an
/// error page or a truncated index parsed as a package is the worst outcome.
pub fn body(mut raw: Vec<u8>) -> Option<Vec<u8>> {
    let (end, length) = framing(&raw)?;
    // The code is the status line's second word, exactly: "HTTP/1.1 500 x
    // 200" is a 500.
    let status = raw[..end].split(|b| *b == b'\n').next()?;
    if status.split(|b| *b == b' ' || *b == b'\r').nth(1) != Some(b"200") {
        return None;
    }
    if let Some(len) = length {
        let want = end.checked_add(len)?;
        if raw.len() < want {
            return None;
        }
        raw.truncate(want);
    }
    raw.drain(..end);
    Some(raw)
}
