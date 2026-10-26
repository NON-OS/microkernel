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

//! A pack's index: its header, entries and names, the pack's first bytes up
//! to its data. A run holds this much of a pack and reads a file's bytes
//! from the store only when it serves them, through `verify`.

use crate::entry::{entry, Entry};
use crate::header::{header, Header};
use crate::refused::Refused;

pub struct Index<'a> {
    pub(crate) head: &'a [u8],
    pub(crate) len: usize,
    pub(crate) header: Header,
}

impl<'a> Index<'a> {
    /// The index of a pack `len` bytes long, from its first bytes: the
    /// header, then exactly up to its data (`data_start`). Every entry,
    /// name and placement is held to the format, and the data area's gaps
    /// are left to `Pack::parse`, which has the bytes: nothing in a gap is
    /// ever served.
    pub fn parse(head: &'a [u8], len: usize) -> Result<Self, Refused> {
        let header = header(head, len)?;
        if head.len() != header.data {
            return Err(Refused::Layout);
        }
        let index = Index { head, len, header };
        crate::layout::placed(&index, None)?;
        Ok(index)
    }

    /// How much of a pack its index is, from the pack's first 64 bytes.
    pub fn data_start(first: &[u8], len: usize) -> Result<usize, Refused> {
        header(first, len).map(|h| h.data)
    }

    pub fn entry(&self, i: usize) -> Result<Entry<'a>, Refused> {
        if i >= self.header.count {
            return Err(Refused::Layout);
        }
        entry(self.head, self.len, &self.header, i)
    }
}
