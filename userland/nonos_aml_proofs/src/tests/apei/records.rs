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

//! Error status blocks, their entries and section data (ACPI 6.5,
//! 18.3.2.7.1; UEFI 2.10, N.2).

/// One entry: its section GUID, severity, revision, and data.
pub fn entry(section: &[u8; 16], severity: u32, revision: u16, data: &[u8]) -> Vec<u8> {
    let mut e = section.to_vec();
    e.extend_from_slice(&severity.to_le_bytes());
    e.extend_from_slice(&revision.to_le_bytes());
    e.extend_from_slice(&[0, 0]);
    e.extend_from_slice(&(data.len() as u32).to_le_bytes());
    e.resize(if revision >= 0x300 { 72 } else { 64 }, 0);
    [e, data.to_vec()].concat()
}

pub fn block(status: u32, severity: u32, entries: &[Vec<u8>]) -> Vec<u8> {
    let data = entries.concat();
    let mut b = status.to_le_bytes().to_vec();
    b.extend_from_slice(&[0; 8]);
    b.extend_from_slice(&(data.len() as u32).to_le_bytes());
    b.extend_from_slice(&severity.to_le_bytes());
    [b, data].concat()
}

/// Section data of `len` bytes with these validation bits and fields.
pub fn data(len: usize, valid: u64, fields: &[(usize, &[u8])]) -> Vec<u8> {
    let mut d = vec![0u8; len];
    d[..8].copy_from_slice(&valid.to_le_bytes());
    fields.iter().for_each(|(at, v)| d[*at..*at + v.len()].copy_from_slice(v));
    d
}
