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

//! An 8.3 name from a path component. The eleven bytes are upper case, as
//! the format requires; the two lowercase flags in the slot's reserved byte
//! are how a listing shows `kernel.bin` while every FAT driver still
//! matches the path case-insensitively. A part in mixed case cannot be
//! spelled without a long-name entry, and this writer refuses it instead of
//! silently changing the name.

use super::part::{part, LOWER_BASE, LOWER_EXT};

pub struct ShortName {
    pub bytes: [u8; 11],
    pub nt_flags: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NameError {
    Empty,
    TooLong,
    BadChar,
    MixedCase,
}

pub fn encode(name: &str) -> Result<ShortName, NameError> {
    let (base, ext) = match name.rfind('.') {
        Some(i) => (&name[..i], &name[i + 1..]),
        None => (name, ""),
    };
    let mut bytes = [b' '; 11];
    let mut flags = 0u8;
    flags |= part(base, &mut bytes[..8], LOWER_BASE)?;
    if !ext.is_empty() {
        flags |= part(ext, &mut bytes[8..], LOWER_EXT)?;
    }
    if base.is_empty() {
        return Err(NameError::Empty);
    }
    Ok(ShortName { bytes, nt_flags: flags })
}
