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

use super::error::FdtError;
use super::header::{self, FdtHeader};
use super::layout;
use super::validate;
use super::walker::Walker;

/// A blob that has passed every structural check. Construction walks the
/// whole structure block once, so a tree that is truncated, unbalanced,
/// nested past `MAX_DEPTH` or has more than one root never becomes an `Fdt`,
/// and nothing downstream has to tell a short walk from a finished one.
pub struct Fdt<'a> {
    pub header: FdtHeader,
    pub blob: &'a [u8],
    pub rsvmap: &'a [u8],
    pub structure: &'a [u8],
    pub strings: &'a [u8],
}

impl<'a> Fdt<'a> {
    /// The tree in `bytes`, which may run past `totalsize`; the excess is
    /// ignored.
    pub fn from_bytes(bytes: &'a [u8]) -> Result<Self, FdtError> {
        let header = header::parse(bytes)?;
        let l = layout::check(&header, bytes.len())?;
        let blob = &bytes[..l.total];
        let fdt = Self {
            header,
            blob,
            rsvmap: &blob[l.rsvmap],
            structure: &blob[l.structure],
            strings: &blob[l.strings],
        };
        validate::structure(fdt.structure, fdt.strings)?;
        Ok(fdt)
    }

    pub fn walker(&self) -> Walker<'a> {
        Walker::new(self.structure, self.strings)
    }

    /// A walk that resumes at a node's body with `depth` nodes open.
    pub fn walker_at(&self, body: usize, depth: usize) -> Walker<'a> {
        Walker::at(self.structure, self.strings, body, depth)
    }
}
