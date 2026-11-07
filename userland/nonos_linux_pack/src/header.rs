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

//! The 64-byte header: where the index, the names and the data are, each
//! exactly where the layout puts it, so one pack has one reading.

use crate::format::{
    u16_at, u32_at, u64_at, up, DATA_ALIGN, DIGEST_LEN, ENTRY_LEN, HEADER_LEN, MAGIC, MAGIC_V1,
    MAX_ENTRIES, MAX_PACK, VERSION, VERSION_V1,
};
use crate::refused::Refused;

#[derive(Clone, Copy, Debug)]
pub struct Header {
    pub version: u16,
    pub count: usize,
    pub names: (usize, usize),
    /// The page digests, right after the names; empty in version 1.
    pub pages: (usize, usize),
    pub data: usize,
}

/// `b` holds at least the header; `len` is the whole pack's length.
pub fn header(b: &[u8], len: usize) -> Result<Header, Refused> {
    if b.len() < HEADER_LEN || u16_at(b, 10) != 0 {
        return Err(Refused::Magic);
    }
    let version = match (&b[..8], u16_at(b, 8)) {
        (m, VERSION_V1) if m == MAGIC_V1 => VERSION_V1,
        (m, VERSION) if m == MAGIC => VERSION,
        _ => return Err(Refused::Magic),
    };
    if len > MAX_PACK || u32_at(b, 12) as usize > MAX_ENTRIES {
        return Err(Refused::TooLarge);
    }
    let count = u32_at(b, 12) as usize;
    let word = |at| usize::try_from(u64_at(b, at)).map_err(|_| Refused::Layout);
    let (index_off, index_len, names_off) = (word(16)?, word(24)?, word(32)?);
    let (names_len, data_off, pages_len) = (word(40)?, word(48)?, word(56)?);
    let index_end = HEADER_LEN + count * ENTRY_LEN;
    let names_end = index_end.checked_add(names_len).ok_or(Refused::Layout)?;
    let pages_end = names_end.checked_add(pages_len).ok_or(Refused::Layout)?;
    let pages_ok = match version {
        VERSION_V1 => pages_len == 0,
        _ => pages_len % DIGEST_LEN == 0,
    };
    let laid_out = index_off == HEADER_LEN
        && index_len == count * ENTRY_LEN
        && names_off == index_end
        && pages_ok
        && Some(data_off) == up(pages_end, DATA_ALIGN)
        && data_off <= len;
    if !laid_out {
        return Err(Refused::Layout);
    }
    Ok(Header {
        version,
        count,
        names: (names_off, names_end),
        pages: (names_end, pages_end),
        data: data_off,
    })
}
