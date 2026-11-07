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

//! Where everything in a pack must be: each name right after the last in
//! the names block, each file's bytes at the one place the builder puts
//! them, a version 2 file's page digests right after the last file's,
//! every byte between zero, and nothing past the last file.

use crate::format::{up, DIGEST_LEN, VERSION_V1};
use crate::index::Index;
use crate::kind::Kind;
use crate::refused::Refused;

/// `data`, the whole pack when there is one to check the data's gaps in.
pub fn placed(index: &Index<'_>, data: Option<&[u8]>) -> Result<(), Refused> {
    let (head, header) = (index.head, index.header);
    let (mut name_at, mut end, mut page) = (header.names.0, header.data, 0);
    let mut last: Option<&[u8]> = None;
    for i in 0..header.count {
        let e = index.entry(i)?;
        if last.is_some_and(|l| l >= e.name) {
            return Err(Refused::Order);
        }
        last = Some(e.name);
        // Each name follows the last in the names block.
        if e.name.as_ptr() as usize - head.as_ptr() as usize != name_at {
            return Err(Refused::Layout);
        }
        name_at += e.name.len();
        let paged = header.version != VERSION_V1 && e.kind == Kind::File;
        // Each version 2 file's digests follow the last file's.
        if paged && e.pages.start != page {
            return Err(Refused::Layout);
        }
        page = page.max(e.pages.end);
        if e.kind != Kind::Dir {
            let align = crate::format::align(paged, e.data.len());
            let start = match e.data.is_empty() {
                true => header.data,
                false => up(end, align).ok_or(Refused::Data)?,
            };
            let gap = data.map_or(&[][..], |b| &b[end.min(start)..start]);
            if e.data.start != start || !zero(gap) {
                return Err(Refused::Data);
            }
            end = end.max(e.data.end);
        }
    }
    let names_whole = name_at == header.names.1 && zero(&head[header.pages.1..header.data]);
    let table_whole = page * DIGEST_LEN == header.pages.1 - header.pages.0;
    // Not a byte past the last file's: one pack has one length.
    if !names_whole || !table_whole || end != index.len {
        return Err(Refused::Layout);
    }
    Ok(())
}

fn zero(b: &[u8]) -> bool {
    b.iter().all(|&x| x == 0)
}
