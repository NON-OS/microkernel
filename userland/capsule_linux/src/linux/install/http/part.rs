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

//! One reply to a GET for a pinned content tar, read as a part of it: a 200
//! from its start, or a 206 from the byte already held (resume.rs). A reply
//! may stop short, as a stream that breaks leaves it. Pure, so the host
//! proofs hold it.

use alloc::vec::Vec;

use super::super::http_reply::framing;

/// What one reply adds: its bytes, the byte they start at, the whole length.
pub struct Part {
    pub from: usize,
    pub total: usize,
    pub bytes: Vec<u8>,
}

/// `raw` as a part, when `have` bytes are held. A 200 starts at 0, and its
/// Content-Length is the whole; a 206 is taken only when its Content-Range
/// starts at `have` and runs to the last byte. Anything else is nothing,
/// and no byte past the whole is kept.
pub fn part(mut raw: Vec<u8>, have: usize) -> Option<Part> {
    let (end, length) = framing(&raw)?;
    let status = raw[..end].split(|b| *b == b'\n').next()?;
    let (from, total) = match status.split(|b| *b == b' ' || *b == b'\r').nth(1)? {
        b"200" => (0, length?),
        b"206" => range(&raw[..end]).filter(|(f, _)| *f == have)?,
        _ => return None,
    };
    raw.drain(..end);
    raw.truncate(length.unwrap_or(usize::MAX).min(total.checked_sub(from)?));
    Some(Part { from, total, bytes: raw })
}

/// `Content-Range: bytes F-L/T` as (F, T), when L is the last byte of T.
fn range(head: &[u8]) -> Option<(usize, usize)> {
    const NAME: &[u8] = b"content-range:";
    let line = head
        .split(|b| *b == b'\n')
        .find(|l| l.len() > NAME.len() && l[..NAME.len()].eq_ignore_ascii_case(NAME))?;
    let value = core::str::from_utf8(&line[NAME.len()..]).ok()?.trim();
    let (span, total) = value.strip_prefix("bytes ")?.split_once('/')?;
    let (first, last) = span.split_once('-')?;
    let (first, last, total): (usize, usize, usize) =
        (first.parse().ok()?, last.parse().ok()?, total.parse().ok()?);
    (first <= last && last.checked_add(1) == Some(total)).then_some((first, total))
}
