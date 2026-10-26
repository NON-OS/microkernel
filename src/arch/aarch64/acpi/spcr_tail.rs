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

//! SPCR's Generic Address Structure for the port's registers (ACPI 6.5,
//! 5.2.3.2), and revision 4's namespace path, by its own length and offset.

use super::le::{u16_at, u64_at, u8_at};
use super::sdt::TableError;
use alloc::vec::Vec;

pub(crate) const FIXED_V4: usize = 88;

pub(crate) fn gas(t: &[u8], at: usize) -> Option<Gas> {
    Some(Gas {
        space: u8_at(t, at)?,
        bit_width: u8_at(t, at + 1)?,
        bit_offset: u8_at(t, at + 2)?,
        access_size: u8_at(t, at + 3)?,
        address: u64_at(t, at + 4)?,
    })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Gas {
    pub space: u8,
    pub bit_width: u8,
    pub bit_offset: u8,
    pub access_size: u8,
    pub address: u64,
}

/// The path, its length and offset at 84 and 86, NUL-terminated inside.
pub(crate) fn namespace(t: &[u8]) -> Result<Vec<u8>, TableError> {
    let (len, at) = (u16_at(t, 84).unwrap_or(0) as usize, u16_at(t, 86).unwrap_or(0) as usize);
    if len == 0 {
        return Ok(Vec::new());
    }
    let bad = TableError::Structure(at);
    let p = t.get(at..at.checked_add(len).ok_or(bad)?).filter(|_| at >= FIXED_V4).ok_or(bad)?;
    Ok(p.split(|c| *c == 0).next().unwrap_or(&[]).to_vec())
}
