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

use super::super::gic::NEEDLES as GIC;
use super::device::PlatformIrq;

/// Flags cell triggers a GIC SPI can be: rising edge and active-high level.
/// The GIC binding allows no other for an SPI; falling or low is a tree
/// for some other controller and is refused, not configured as its opposite.
const EDGE_RISING: u32 = 1;
const LEVEL_HIGH: u32 = 4;

/// The node's first interrupt when it is a GIC SPI. A line to another
/// controller (the Pi's legacy ARMC) is not one the broker can route, and a
/// PPI is a CPU's, not a device's.
pub(super) fn first_spi(fdt: &Fdt, node: &NodeRef) -> Result<Option<PlatformIrq>, FdtError> {
    let Some(spec) = fdt.interrupt(node, 0)? else {
        return Ok(None);
    };
    if fdt.compatible_with(&spec.controller, GIC)?.is_none() {
        return Ok(None);
    }
    let intid = gic_intid(spec.cells)?;
    if intid < 32 {
        return Ok(None);
    }
    let edge = match be_u32(spec.cells, 8)? & 0xf {
        EDGE_RISING => true,
        LEVEL_HIGH => false,
        _ => return Err(FdtError::BadCells),
    };
    Ok(Some(PlatformIrq { intid, edge }))
}
