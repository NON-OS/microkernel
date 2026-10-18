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

//! The header every table starts with (ACPI 6.5, 5.2.6): the signature,
//! a length no longer than the bytes handed over, and a checksum that sums
//! the table to zero. A table that fails any of these is not read at all.

use super::le::{u32_at, u8_at};

pub const HEADER: usize = 36;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TableError {
    /// Fewer bytes than a header, or than the table's fixed part.
    Short,
    Signature,
    /// The length field says more than was handed over.
    Length,
    Checksum,
    /// A structure at this offset runs past the table, or is too short for
    /// its type.
    Structure(usize),
    /// A reference at this offset names nothing it may name, or a chain of
    /// them loops.
    Reference(usize),
}

/// The table, cut to its own length.
pub fn table<'a>(t: &'a [u8], signature: &[u8; 4]) -> Result<&'a [u8], TableError> {
    if t.len() < HEADER {
        return Err(TableError::Short);
    }
    if &t[..4] != signature {
        return Err(TableError::Signature);
    }
    let len = u32_at(t, 4).ok_or(TableError::Short)? as usize;
    let t = t.get(..len).ok_or(TableError::Length)?;
    if len < HEADER {
        return Err(TableError::Short);
    }
    if t.iter().fold(0u8, |s, b| s.wrapping_add(*b)) != 0 {
        return Err(TableError::Checksum);
    }
    Ok(t)
}

pub fn revision(t: &[u8]) -> u8 {
    u8_at(t, 8).unwrap_or(0)
}
