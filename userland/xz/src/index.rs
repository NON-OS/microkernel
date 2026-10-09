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

//! The index at a stream's end: one record per block, which must agree with
//! what was decoded, then padding and a CRC-32.

use super::block::Sizes;
use super::crc32::matches;
use super::varint::varint;

/// The bytes the index took, if it describes exactly `blocks`.
pub fn index(d: &[u8], blocks: &[Sizes]) -> Option<usize> {
    if *d.first()? != 0 {
        return None;
    }
    let (count, n) = varint(d.get(1..)?)?;
    if count != blocks.len() as u64 {
        return None;
    }
    let mut at = 1 + n;
    for b in blocks {
        let (unpadded, n) = varint(d.get(at..)?)?;
        let (uncompressed, m) = varint(d.get(at + n..)?)?;
        if unpadded != b.unpadded || uncompressed != b.uncompressed {
            return None;
        }
        at += n + m;
    }
    while at % 4 != 0 {
        if *d.get(at)? != 0 {
            return None;
        }
        at += 1;
    }
    matches(&d[..at], d.get(at..at + 4)?).then_some(at + 4)
}
