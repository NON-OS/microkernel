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
use crate::arch::fdt::irq::gic_intid;
use crate::arch::fdt::node::NodeRef;
use crate::arch::fdt::parser::Fdt;

use super::intx::IntxRoute;

/// Interrupt controllers whose three-cell specifier `gic_intid` reads.
const GICS: &[&[u8]] =
    &[b"arm,gic-v3", b"arm,gic-400", b"arm,cortex-a15-gic", b"arm,cortex-a7-gic"];

/// One entry and its length in bytes: child address (3 cells), pin, the
/// parent's phandle, its unit address and its three-cell specifier.
pub(super) fn entry(fdt: &Fdt, data: &[u8]) -> Result<(IntxRoute, usize), FdtError> {
    let gic = fdt.by_phandle(be_u32(data, 16)?)?;
    if fdt.compatible_with(&gic, GICS)?.is_none() || fdt.interrupt_cells(&gic)? != 3 {
        return Err(FdtError::BadCells);
    }
    let unit = parent_address_cells(fdt, &gic)? as usize * 4;
    let spec = data.get(20 + unit..32 + unit).ok_or(FdtError::OutOfBounds)?;
    let intid = gic_intid(spec)?;
    let edge = match be_u32(spec, 8)? {
        4 => false,
        1 => true,
        _ => return Err(FdtError::BadCells),
    };
    if intid < 32 {
        return Err(FdtError::BadCells);
    }
    let route = IntxRoute { child_hi: be_u32(data, 0)?, pin: be_u32(data, 12)?, intid, edge };
    Ok((route, 32 + unit))
}

/// The parent's unit address is `#address-cells` of the parent node
/// itself, zero when it has none.
fn parent_address_cells(fdt: &Fdt, gic: &NodeRef) -> Result<u32, FdtError> {
    match fdt.property(gic, b"#address-cells")? {
        Some(p) => p.cell_count(),
        None => Ok(0),
    }
}
