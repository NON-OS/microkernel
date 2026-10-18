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

//! PkgLength, section 20.2.4.
//!
//! The lead byte's top two bits count the bytes that follow it (0 to 3). With
//! none, its low six bits are the length. With some, its low four bits are the
//! least significant nibble, bits 4 and 5 must be zero, and each following
//! byte adds eight more bits. The length counts its own bytes too, so it is
//! measured from the lead byte.

use super::Cursor;
use crate::error::{Error, ErrorKind};

/// Reads a PkgLength and returns the table offset its package ends at,
/// refusing a length too short to hold its own encoding and a package reaching
/// past the one around it.
pub fn pkg_end(c: &mut Cursor<'_>) -> Result<u32, Error> {
    let start = c.pos;
    let len = pkg_value(c)?;
    if len < c.pos - start {
        return Err(Error::new(ErrorKind::PkgLength, start));
    }
    let end = start.checked_add(len).ok_or(Error::new(ErrorKind::PkgLength, start))?;
    if end > c.end {
        return Err(Error::new(ErrorKind::PastParent, start));
    }
    Ok(end)
}

/// The encoded value alone, with reserved bits refused. A field element's
/// width uses this encoding for a count of bits rather than a span of bytes
/// (section 20.2.5.2, NamedField), so it is read without the checks above.
pub fn pkg_value(c: &mut Cursor<'_>) -> Result<u32, Error> {
    let start = c.pos;
    let lead = c.u8()?;
    let follow = (lead >> 6) as u32;
    if follow == 0 {
        return Ok((lead & 0x3F) as u32);
    }
    if lead & 0x30 != 0 {
        return Err(Error::new(ErrorKind::PkgLength, start));
    }
    let mut v = (lead & 0x0F) as u32;
    for i in 0..follow {
        v |= (c.u8()? as u32) << (4 + 8 * i);
    }
    Ok(v)
}
