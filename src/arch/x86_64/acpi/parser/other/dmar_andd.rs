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

//! ACPI namespace device declarations (DMAR type 4, VT-d 3.x section 8.5).
//! A device firmware describes only in the ACPI namespace, such as an SD
//! host or an LPSS controller in ACPI mode, is named by its object path and
//! given a number that the remapping units' device scopes refer to. Pure.

pub const ANDD_TYPE: u16 = 4;
/// Object path bytes kept; a longer path is refused rather than cut short.
pub const MAX_NAME: usize = 40;
const NUMBER_AT: usize = 7;
const NAME_AT: usize = 8;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Andd {
    pub number: u8,
    pub name: [u8; MAX_NAME],
    pub name_len: u8,
}

impl Andd {
    pub fn name(&self) -> &[u8] {
        &self.name[..self.name_len as usize]
    }
}

/// `bytes` is the whole structure. None when it is not an ANDD, is short, or
/// its name is empty, longer than `MAX_NAME`, not ended by a NUL inside the
/// structure, or holds a byte no namespace path has.
pub fn parse_andd(bytes: &[u8]) -> Option<Andd> {
    if bytes.len() <= NAME_AT || u16::from_le_bytes([bytes[0], bytes[1]]) != ANDD_TYPE {
        return None;
    }
    let raw = &bytes[NAME_AT..];
    let len = raw.iter().position(|b| *b == 0)?;
    if len == 0 || len > MAX_NAME || !raw[..len].iter().all(|b| b.is_ascii_graphic()) {
        return None;
    }
    let mut name = [0u8; MAX_NAME];
    name[..len].copy_from_slice(&raw[..len]);
    Some(Andd { number: bytes[NUMBER_AT], name, name_len: len as u8 })
}
