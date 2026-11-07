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

//! A version 2 file read a page at a time: which pages cover a range of it,
//! where they are in the pack, and each page served only once it is its
//! digest in the head, which the pack's trailer binds.

use core::ops::Range;

use crate::entry::Entry;
use crate::format::{DIGEST_LEN, PAGE, VERSION_V1};
use crate::index::Index;
use crate::kind::Kind;
use crate::refused::Refused;

impl<'a> Index<'a> {
    /// Whether this pack keeps page digests (version 2).
    pub fn paged(&self) -> bool {
        self.header.version != VERSION_V1
    }

    /// The pages of `e` that cover `[offset, offset + len)`, cut at the
    /// file's end, as page numbers within the file; empty past its end or
    /// for a file with no page digests.
    pub fn pages_over(&self, e: &Entry<'a>, offset: usize, len: usize) -> Range<usize> {
        let size = e.data.len();
        if e.pages.is_empty() || offset >= size || len == 0 {
            return 0..0;
        }
        let end = offset.saturating_add(len).min(size);
        offset / PAGE..end.div_ceil(PAGE)
    }

    /// Where page `page` of `e` is in the pack: a whole page, or the file's
    /// last bytes.
    pub fn page_bytes(&self, e: &Entry<'a>, page: usize) -> Option<Range<usize>> {
        if page >= e.pages.len() {
            return None;
        }
        let start = e.data.start + page * PAGE;
        Some(start..(start + PAGE).min(e.data.end))
    }

    /// `bytes`, read from the pack at `page_bytes(e, page)`, once they are
    /// that page's digest.
    pub fn verify_page<'b>(
        &self,
        e: &Entry<'a>,
        page: usize,
        bytes: &'b [u8],
    ) -> Result<&'b [u8], Refused> {
        let at = self.page_bytes(e, page).ok_or(Refused::Digest)?;
        let i = e.pages.start + page;
        let slot = self.header.pages.0 + i * DIGEST_LEN;
        let want = self.head.get(slot..slot + DIGEST_LEN).ok_or(Refused::Digest)?;
        let ok = e.kind == Kind::File
            && bytes.len() == at.len()
            && blake3::hash(bytes).as_bytes()[..DIGEST_LEN] == *want;
        match ok {
            true => Ok(bytes),
            false => Err(Refused::Digest),
        }
    }
}
