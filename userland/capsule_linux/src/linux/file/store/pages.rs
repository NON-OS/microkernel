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

//! A range of a packed file put together from its pages: those already
//! held, and the rest fetched in runs, one fetch a run of pages not held.
//! What `fetch` returns is checked there (pack/serve.rs `read_pages`); this
//! only places it.

use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use core::ops::Range;

use nonos_linux_pack::PAGE;

/// The pages of a file `size` bytes long that cover `[offset, offset + len)`.
pub fn pages_over(size: u64, offset: u64, len: u32) -> Range<usize> {
    let end = offset.saturating_add(u64::from(len)).min(size);
    if offset >= end {
        return 0..0;
    }
    let page = PAGE as u64;
    (offset / page) as usize..end.div_ceil(page) as usize
}

/// Up to `len` bytes from `offset` of a file `size` bytes long. A fetched
/// page is kept in `got`; a page of the wrong length is refused, never cut
/// or padded.
pub fn assemble<E: Copy>(
    size: u64,
    offset: u64,
    len: u32,
    got: &mut BTreeMap<usize, Vec<u8>>,
    mut fetch: impl FnMut(Range<usize>) -> Result<Vec<Vec<u8>>, E>,
    short: E,
) -> Result<Vec<u8>, E> {
    let want = pages_over(size, offset, len);
    let mut page = want.start;
    while page < want.end {
        if got.contains_key(&page) {
            page += 1;
            continue;
        }
        let run_end = (page..want.end).find(|p| got.contains_key(p)).unwrap_or(want.end);
        let read = fetch(page..run_end)?;
        if read.len() != run_end - page {
            return Err(short);
        }
        for (n, bytes) in read.into_iter().enumerate() {
            let p = page + n;
            let base = (p * PAGE) as u64;
            if bytes.len() as u64 != (size - base).min(PAGE as u64) {
                return Err(short);
            }
            got.insert(p, bytes);
        }
        page = run_end;
    }
    let end = offset.saturating_add(u64::from(len)).min(size);
    let mut out = Vec::with_capacity(end.saturating_sub(offset) as usize);
    for p in want {
        let held = got.get(&p).ok_or(short)?;
        let base = (p * PAGE) as u64;
        let from = (offset.max(base) - base) as usize;
        let to = (end.min(base + held.len() as u64) - base) as usize;
        out.extend_from_slice(&held[from..to]);
    }
    Ok(out)
}
