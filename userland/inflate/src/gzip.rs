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
//! gzip, including the concatenated members RFC 1952 allows.

use alloc::vec::Vec;

use super::crc32::crc32;
use super::gzip_header::body_at;
use super::inflate_raw::inflate_counted;
use super::tables::MAX_OUT;

/// A member ends with a CRC32 and an ISIZE.
const TRAILER: usize = 8;

/// Enough for a distribution index in several parts.
pub(super) const MAX_MEMBERS: usize = 64;

/// Every member, concatenated. Bytes after a verified member that do not
/// decode as another member are trailing garbage, and end the stream.
pub fn gunzip(data: &[u8]) -> Option<Vec<u8>> {
    let (mut out, mut at) = verified(data)?;
    for _ in 1..MAX_MEMBERS {
        let rest = data.get(at..)?;
        if rest.is_empty() {
            return Some(out);
        }
        let Some((mut part, end)) = inflated(rest) else {
            return Some(out);
        };
        checked(rest, &part, end)?;
        if out.len().checked_add(part.len())? > MAX_OUT {
            return None;
        }
        out.append(&mut part);
        at = at.checked_add(end)?.checked_add(TRAILER)?;
    }
    (at == data.len()).then_some(out)
}

pub(super) fn verified(d: &[u8]) -> Option<(Vec<u8>, usize)> {
    let (out, end) = inflated(d)?;
    checked(d, &out, end)?;
    Some((out, end.checked_add(TRAILER)?))
}

/// The member's output, and the offset of its trailer.
fn inflated(d: &[u8]) -> Option<(Vec<u8>, usize)> {
    let start = body_at(d)?;
    let (out, used) = inflate_counted(d.get(start..)?)?;
    Some((out, start.checked_add(used)?))
}

fn checked(d: &[u8], out: &[u8], end: usize) -> Option<()> {
    let t = d.get(end..end.checked_add(TRAILER)?)?;
    let word = |i: usize| u32::from_le_bytes([t[i], t[i + 1], t[i + 2], t[i + 3]]);
    (word(0) == crc32(out) && word(4) == out.len() as u32).then_some(())
}
