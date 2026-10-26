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

//! A whole pack, held to the format before anything of it is served: its
//! index (index.rs), and every gap in its data zero, so no two files share
//! a byte and one pack has one layout.

use core::ops::Deref;

use crate::entry::Entry;
use crate::header::header;
use crate::index::Index;
use crate::refused::Refused;

pub struct Pack<'a> {
    pub(crate) bytes: &'a [u8],
    index: Index<'a>,
}

impl<'a> Pack<'a> {
    pub fn parse(bytes: &'a [u8]) -> Result<Self, Refused> {
        let data = header(bytes, bytes.len())?.data;
        let index = Index::parse(&bytes[..data], bytes.len())?;
        crate::layout::placed(&index, Some(bytes))?;
        Ok(Pack { bytes, index })
    }

    /// A file's bytes, or a link's target, once they are its digest.
    pub fn file(&self, e: &Entry<'a>) -> Result<&'a [u8], Refused> {
        self.index.verify(e, &self.bytes[e.data.clone()])
    }
}

impl<'a> Deref for Pack<'a> {
    type Target = Index<'a>;

    fn deref(&self) -> &Index<'a> {
        &self.index
    }
}
