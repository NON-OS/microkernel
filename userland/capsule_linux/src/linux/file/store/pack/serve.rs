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

//! Serving a guest-visible path out of the packs: whether it is there, and
//! its bytes. A path the packs do not hold is None, and
//! the store's own tree answers it instead.

use alloc::vec::Vec;
use core::ops::Range;

use nonos_linux_pack::Kind;

use super::table::with;

/// A visible path as a pack names it: no leading `/`.
pub(super) fn name(visible: &[u8]) -> &[u8] {
    visible.strip_prefix(b"/").unwrap_or(visible)
}

/// Its size and whether it is a directory: one a pack names, or one with a
/// pack's files below it.
pub fn stat(visible: &[u8]) -> Option<(u64, bool)> {
    let n = name(visible);
    with(|packs| {
        packs.iter().find_map(|p| match p.index.find(n) {
            Some(e) => Some((e.data.len() as u64, e.kind == Kind::Dir)),
            None if !n.is_empty() && !p.index.under(n).is_empty() => Some((0, true)),
            None => None,
        })
    })
}

/// Its bytes, up to `max`, read whole and served only once they are its
/// digest: a cut read is never checked, so it is never served.
pub fn read(visible: &[u8], max: u32) -> Option<Result<Vec<u8>, &'static str>> {
    let n = name(visible);
    with(|packs| {
        let p = packs.iter_mut().find(|p| p.index.find(n).is_some_and(|e| e.kind == Kind::File))?;
        let e = p.index.find(n)?;
        let read = p.stream.read_window(e.data.start as u64, u32::try_from(e.data.len()).ok()?);
        Some(read.and_then(|mut bytes| match p.index.verify(&e, &bytes).is_ok() {
            true => Ok({
                bytes.truncate(max as usize);
                bytes
            }),
            false => Err("a packed file is not its digest"),
        }))
    })
}

/// A file a version 2 pack holds with page digests: its size and its whole
/// digest, which names it to `read_pages` for as long as it is open.
pub fn paged(visible: &[u8]) -> Option<(u64, [u8; 16])> {
    let n = name(visible);
    with(|packs| {
        let p = packs.iter().find(|p| p.index.find(n).is_some_and(|e| e.kind == Kind::File))?;
        let e = p.index.find(n)?;
        (p.index.paged() && !e.pages.is_empty()).then(|| (e.data.len() as u64, e.digest))
    })
}

/// Pages `pages` of the file `paged` named, read from its pack in one
/// window and each served only once it is its digest. Err when the pack
/// holds another file under that name now, or a page is not its digest.
pub fn read_pages(
    visible: &[u8],
    digest: &[u8; 16],
    pages: Range<usize>,
) -> Result<Vec<Vec<u8>>, &'static str> {
    let n = name(visible);
    with(|packs| {
        let p = packs.iter_mut().find(|p| p.index.find(n).is_some_and(|e| e.kind == Kind::File));
        let p = p.ok_or("the packed file is gone")?;
        let e = p.index.find(n).ok_or("the packed file is gone")?;
        if e.digest != *digest || pages.end > e.pages.len() || pages.is_empty() {
            return Err("the packed file changed");
        }
        let first = p.index.page_bytes(&e, pages.start).ok_or("no such page")?;
        let last = p.index.page_bytes(&e, pages.end - 1).ok_or("no such page")?;
        let len = u32::try_from(last.end - first.start).map_err(|_| "too large")?;
        let window = p.stream.read_window(first.start as u64, len)?;
        let mut out = Vec::with_capacity(pages.len());
        for page in pages {
            let at = p.index.page_bytes(&e, page).ok_or("no such page")?;
            let bytes =
                window.get(at.start - first.start..at.end - first.start).ok_or("a cut read")?;
            let ok = p
                .index
                .verify_page(&e, page, bytes)
                .map_err(|_| "a packed page is not its digest")?;
            out.push(ok.to_vec());
        }
        crate::linux::start_say::clock::read(window.len());
        Ok(out)
    })
}
