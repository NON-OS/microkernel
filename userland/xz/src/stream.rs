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

//! Streams, one after another with zero padding between: header, blocks,
//! index, footer. The footer must agree with the header and the index.

use alloc::vec::Vec;

use super::block::block;
use super::check::Check;
use super::crc32::matches;
use super::index::index;
use super::limits::MAX_OUT;

const MAGIC: [u8; 6] = [0xFD, 0x37, 0x7A, 0x58, 0x5A, 0x00];
const FOOTER_MAGIC: [u8; 2] = *b"YZ";

pub fn decompress(data: &[u8]) -> Option<Vec<u8>> {
    let (mut out, mut at) = (Vec::new(), 0usize);
    loop {
        let h = data.get(at..at + 12)?;
        let flags = [h[6], h[7]];
        if h[..6] != MAGIC || flags[0] != 0 || !matches(&flags, &h[8..12]) {
            return None;
        }
        let check = Check::from_flag(flags[1])?;
        let mut p = at + 12;
        let mut blocks = Vec::new();
        while *data.get(p)? != 0 {
            let (used, sizes) = block(&data[p..], check, &mut out)?;
            blocks.push(sizes);
            p += used;
            if out.len() > MAX_OUT {
                return None;
            }
        }
        let size = index(&data[p..], &blocks)?;
        p += size;
        let f = data.get(p..p + 12)?;
        let backward = (u64::from(u32::from_le_bytes([f[4], f[5], f[6], f[7]])) + 1) * 4;
        if !matches(&f[4..10], &f[..4])
            || backward != size as u64
            || f[8..10] != flags
            || f[10..] != FOOTER_MAGIC
        {
            return None;
        }
        at = p + 12;
        while data.get(at..at + 4) == Some(&[0, 0, 0, 0]) {
            at += 4;
        }
        if at == data.len() {
            return Some(out);
        }
    }
}
