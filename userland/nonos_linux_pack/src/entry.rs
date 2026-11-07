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

//! One 48-byte index entry, read and held to the format: its name in the
//! names block, its kind and mode, and its bytes in the data area.

use core::ops::Range;

use crate::format::{pages, u16_at, u32_at, u64_at, DIGEST_LEN, ENTRY_LEN};
use crate::format::{HEADER_LEN, TARGET_MAX, VERSION_V1};
use crate::header::Header;
use crate::kind::Kind;
use crate::refused::Refused;

/// A file, a directory, or a symbolic link and its target (`data`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry<'a> {
    pub name: &'a [u8],
    pub kind: Kind,
    pub exec: bool,
    pub data: Range<usize>,
    pub digest: [u8; DIGEST_LEN],
    /// A version 2 file's page digests, as indices into the head's table;
    /// empty for a directory, a link, an empty file and every version 1
    /// entry.
    pub pages: Range<usize>,
}

/// `b` is the pack's head, its first `h.data` bytes; `pack_len`, its length.
pub fn entry<'a>(b: &'a [u8], pack_len: usize, h: &Header, i: usize) -> Result<Entry<'a>, Refused> {
    let e = &b[HEADER_LEN + i * ENTRY_LEN..][..ENTRY_LEN];
    let (off, len) = (u32_at(e, 0) as usize, u16_at(e, 4) as usize);
    let name = b.get(off..off + len).filter(|_| off >= h.names.0 && off + len <= h.names.1);
    let name = name.filter(|n| crate::name::valid(n)).ok_or(Refused::Name)?;
    let (kind, exec) = crate::kind::kind(e[6], e[7])?;
    let word = |at| usize::try_from(u64_at(e, at)).map_err(|_| Refused::Data);
    let (start, n, first) = (word(8)?, word(16)?, word(40)?);
    let end = start.checked_add(n).ok_or(Refused::Data)?;
    let paged = h.version != VERSION_V1 && kind == Kind::File;
    let table = (h.pages.1 - h.pages.0) / DIGEST_LEN;
    let page_list = match paged {
        true => first..first.checked_add(pages(n)).ok_or(Refused::Data)?,
        false => 0..0,
    };
    // Version 1 has no table, and only a version 2 file names a page in it.
    if (!paged && first != 0) || page_list.end > table {
        return Err(Refused::Layout);
    }
    let align = crate::format::align(paged, n);
    let placed = match kind {
        Kind::Dir => start == 0 && n == 0,
        Kind::File => start >= h.data && start % align == 0 && end <= pack_len,
        Kind::Link => start >= h.data && end <= pack_len && (1..=TARGET_MAX).contains(&n),
    };
    if !placed {
        return Err(Refused::Data);
    }
    let mut digest = [0u8; DIGEST_LEN];
    digest.copy_from_slice(&e[24..24 + DIGEST_LEN]);
    // A directory has no bytes, so no digest: its field is zero.
    if kind == Kind::Dir && digest != [0; DIGEST_LEN] {
        return Err(Refused::Data);
    }
    Ok(Entry { name, kind, exec, data: start..end, digest, pages: page_list })
}
