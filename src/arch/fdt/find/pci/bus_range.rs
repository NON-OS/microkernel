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

use crate::arch::fdt::endian::be_u32;
use crate::arch::fdt::error::FdtError;
use crate::arch::fdt::node::NodeRef;
use crate::arch::fdt::parser::Fdt;

/// The bridge's `bus-range`, or the whole 0 to 255 the binding defaults to.
/// Two cells, first no greater than last, each a bus number.
pub fn bus_range(fdt: &Fdt, node: &NodeRef) -> Result<(u8, u8), FdtError> {
    let Some(prop) = fdt.property(node, b"bus-range")? else {
        return Ok((0, 255));
    };
    if prop.data.len() != 8 {
        return Err(FdtError::BadCells);
    }
    let first = be_u32(prop.data, 0)?;
    let last = be_u32(prop.data, 4)?;
    match (u8::try_from(first), u8::try_from(last)) {
        (Ok(first), Ok(last)) if first <= last => Ok((first, last)),
        _ => Err(FdtError::BadCells),
    }
}
