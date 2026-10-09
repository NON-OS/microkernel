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

//! One block: header, LZMA2 data, padding to four bytes, and the check.

use alloc::vec::Vec;

use super::block_header::header;
use super::check::Check;
use super::lzma2::lzma2;

/// What the index must say about a block: its unpadded and output sizes.
pub struct Sizes {
    pub unpadded: u64,
    pub uncompressed: u64,
}

/// Decode a block into `out`; the bytes it took, and its sizes.
pub fn block(d: &[u8], check: Check, out: &mut Vec<u8>) -> Option<(usize, Sizes)> {
    let h = header(d)?;
    let start = out.len();
    let used = lzma2(d.get(h.size..)?, h.dict, out)?;
    let produced = (out.len() - start) as u64;
    if h.compressed.is_some_and(|c| c != used as u64)
        || h.uncompressed.is_some_and(|u| u != produced)
    {
        return None;
    }
    let mut at = h.size + used;
    while at % 4 != 0 {
        if *d.get(at)? != 0 {
            return None;
        }
        at += 1;
    }
    let stored = d.get(at..at + check.size())?;
    if !check.holds(&out[start..], stored) {
        return None;
    }
    let unpadded = (h.size + used + check.size()) as u64;
    Some((at + check.size(), Sizes { unpadded, uncompressed: produced }))
}
