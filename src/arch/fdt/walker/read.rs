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

//! Decoding the payload that follows a BEGIN_NODE or PROP token.

use crate::arch::fdt::endian::be_u32;
use crate::arch::fdt::error::FdtError;
use crate::arch::fdt::property::Property;
use crate::arch::fdt::strings::resolve;

use super::align::align4;

/// The NUL-terminated name at `start`, and the aligned offset after it.
pub(super) fn node_name(structure: &[u8], start: usize) -> Result<(&[u8], usize), FdtError> {
    let rest = structure.get(start..).ok_or(FdtError::TruncatedNodeName)?;
    let len = rest.iter().position(|&b| b == 0).ok_or(FdtError::TruncatedNodeName)?;
    Ok((&rest[..len], align4(start + len + 1)))
}

/// The property whose length and name offset start at `start`, and the
/// aligned offset after its value.
pub(super) fn property<'a>(
    structure: &'a [u8],
    strings: &'a [u8],
    start: usize,
) -> Result<(Property<'a>, usize), FdtError> {
    let len = be_u32(structure, start).map_err(|_| FdtError::TruncatedProperty)? as usize;
    let nameoff = be_u32(structure, start + 4).map_err(|_| FdtError::TruncatedProperty)?;
    let value = start + 8;
    let end = value.checked_add(len).ok_or(FdtError::TruncatedProperty)?;
    let data = structure.get(value..end).ok_or(FdtError::TruncatedProperty)?;
    let name = resolve(strings, nameoff)?;
    Ok((Property { name, data }, align4(end)))
}
