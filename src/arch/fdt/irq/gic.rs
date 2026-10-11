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

//! The ARM GIC binding's interrupt specifier: type, number, flags.

use crate::arch::fdt::endian::be_u32;
use crate::arch::fdt::error::FdtError;

const SPI: u32 = 0;
const PPI: u32 = 1;
/// SPIs run from INTID 32 to 1019; the GIC reserves 1020 and up.
const LAST_SPI: u32 = 987;
const LAST_PPI: u32 = 15;

/// The INTID a GIC specifier names. The number is range checked against its
/// type: a PPI past 15 or an SPI past 987 is a broken tree, and saturating it
/// would ack and route an interrupt nobody raised. The GICv3.1 extended
/// ranges (types 2 and 3) are refused until a board needs them.
pub fn gic_intid(cells: &[u8]) -> Result<u32, FdtError> {
    if cells.len() < 12 {
        return Err(FdtError::BadCells);
    }
    let number = be_u32(cells, 4)?;
    match be_u32(cells, 0)? {
        SPI if number <= LAST_SPI => Ok(32 + number),
        PPI if number <= LAST_PPI => Ok(16 + number),
        _ => Err(FdtError::BadCells),
    }
}

/// The trigger in the flags cell: 1 and 2 are edge, 4 and 8 level. The CPU
/// mask a GICv2 tree puts in bits 8 to 15 of a PPI is not a trigger and is
/// dropped.
pub fn gic_edge(cells: &[u8]) -> Result<bool, FdtError> {
    if cells.len() < 12 {
        return Err(FdtError::BadCells);
    }
    Ok(be_u32(cells, 8)? & 0x3 != 0)
}
