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

//! A list of kernel capability names, as `AUTH_OK`, a grant and the record
//! carry them: a u8 count, then each name as a u8 length and its bytes.
//! Names, not bit numbers, cross the wire, so a renumbered kernel cannot
//! quietly change what a list means. A name is `[A-Z][A-Za-z0-9]*`, and the
//! names come in ascending byte order, so a list never names a capability
//! twice and one set of capabilities has one encoding.

use crate::names;
use crate::{Reader, WireError};

/// There are 40 capabilities (`src/capabilities/types`); a list of more
/// must repeat one.
pub const MAX_CAPS: usize = 40;
pub const MAX_CAP_NAME: usize = 32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapNames<'a> {
    count: u8,
    bytes: &'a [u8],
}

impl<'a> CapNames<'a> {
    pub fn read(r: &mut Reader<'a>) -> Result<Self, WireError> {
        let count = r.u8()?;
        if usize::from(count) > MAX_CAPS {
            return Err(WireError::Oversize);
        }
        let start = r.pos;
        let mut before: Option<&[u8]> = None;
        for _ in 0..count {
            let name = r.bytes8(1, MAX_CAP_NAME)?;
            names::cap(name)?;
            if before.is_some_and(|b| name <= b) {
                return Err(WireError::Value);
            }
            before = Some(name);
        }
        Ok(CapNames { count, bytes: &r.buf[start..r.pos] })
    }

    pub const fn len(&self) -> usize {
        self.count as usize
    }

    pub const fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn iter(&self) -> impl Iterator<Item = &'a [u8]> + 'a {
        let mut r = Reader::new(self.bytes);
        (0..self.count).filter_map(move |_| r.bytes8(1, MAX_CAP_NAME).ok())
    }
}
