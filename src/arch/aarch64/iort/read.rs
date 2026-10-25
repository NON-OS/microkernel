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

//! Little-endian fields read with their bounds checked, as ACPI lays every
//! table out (ACPI specification, the system description tables).

use super::error::IortError;

pub(super) fn bytes<const N: usize>(table: &[u8], at: usize) -> Result<[u8; N], IortError> {
    let end = at.checked_add(N).ok_or(IortError::Short)?;
    let mut out = [0u8; N];
    out.copy_from_slice(table.get(at..end).ok_or(IortError::Short)?);
    Ok(out)
}

pub(super) fn u8_at(table: &[u8], at: usize) -> Result<u8, IortError> {
    Ok(bytes::<1>(table, at)?[0])
}

pub(super) fn u16_at(table: &[u8], at: usize) -> Result<u16, IortError> {
    Ok(u16::from_le_bytes(bytes(table, at)?))
}

pub(super) fn u32_at(table: &[u8], at: usize) -> Result<u32, IortError> {
    Ok(u32::from_le_bytes(bytes(table, at)?))
}

pub(super) fn u64_at(table: &[u8], at: usize) -> Result<u64, IortError> {
    Ok(u64::from_le_bytes(bytes(table, at)?))
}

/// A u32 offset or count as an index.
pub(super) fn index(table: &[u8], at: usize) -> Result<usize, IortError> {
    usize::try_from(u32_at(table, at)?).map_err(|_| IortError::Short)
}
