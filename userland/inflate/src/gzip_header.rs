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

//! The gzip member header (RFC 1952 2.3), and where its deflate stream starts.

use super::types::End;

const MAGIC: [u8; 3] = [0x1f, 0x8b, 8];
const FHCRC: u8 = 2;
const FEXTRA: u8 = 4;
const FNAME: u8 = 8;
const FCOMMENT: u8 = 16;
const RESERVED: u8 = 0xe0;

/// Offset of the first deflate byte. `Truncated` when the input ends
/// inside a header that is valid so far, `Corrupt` when it is no header.
pub(super) fn body_at(d: &[u8]) -> Result<usize, End> {
    let n = d.len().min(3);
    if d[..n] != MAGIC[..n] {
        return Err(End::Corrupt);
    }
    let &flg = d.get(3).ok_or(End::Truncated)?;
    if flg & RESERVED != 0 {
        return Err(End::Corrupt);
    }
    let mut p = 10usize;
    if flg & FEXTRA != 0 {
        let x = d.get(p..p + 2).ok_or(End::Truncated)?;
        p += 2 + usize::from(u16::from_le_bytes([x[0], x[1]]));
    }
    for flag in [FNAME, FCOMMENT] {
        if flg & flag != 0 {
            let zero = d.get(p..).and_then(|r| r.iter().position(|&c| c == 0));
            p += zero.ok_or(End::Truncated)? + 1;
        }
    }
    if flg & FHCRC != 0 {
        p += 2;
    }
    if p >= d.len() {
        return Err(End::Truncated);
    }
    Ok(p)
}
