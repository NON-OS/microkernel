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

use alloc::vec::Vec;

use super::cursor::Cursor;
use super::directory::read_directory;
use super::sfnt;

/// The largest sfnt a WOFF2 may unpack to, and so the most its table
/// stream may decompress to.
const MAX_SFNT: usize = 8 << 20;

/// Unpack a WOFF2 file (W3C WOFF2, 2024) into the sfnt it was made from:
/// decompress the table stream, rebuild the transformed glyf, loca and
/// hmtx tables, and lay out the font. None for anything malformed and
/// for font collections, which a web font source cannot pick from.
pub(crate) fn unwrap_woff2(data: &[u8]) -> Option<Vec<u8>> {
    let mut c = Cursor::new(data);
    let (sig, flavor, length, n) = (c.u32()?, c.u32()?, c.u32()?, c.u16()? as usize);
    if sig != u32::from_be_bytes(*b"wOF2") || &flavor.to_be_bytes() == b"ttcf" {
        return None;
    }
    if length as usize != data.len() || n == 0 {
        return None;
    }
    /* Reserved field and totalSfntSize, then the stream size, then the
     * versions and the metadata and private blocks, not used here. */
    c.bytes(6)?;
    let packed_len = c.u32()? as usize;
    c.bytes(24)?;
    let tables = read_directory(&mut c, n)?;
    let total = tables.last()?.src.end;
    if total > MAX_SFNT {
        return None;
    }
    let raw = nonos_brotli::decompress(c.bytes(packed_len)?, total).ok()?;
    if raw.len() != total {
        return None;
    }
    let mut built = super::tables::rebuild(&tables, &raw)?;
    sfnt::assemble(flavor.to_be_bytes(), &mut built, MAX_SFNT)
}
