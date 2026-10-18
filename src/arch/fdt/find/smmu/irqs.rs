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

use crate::arch::fdt::error::FdtError;
use crate::arch::fdt::irq::{gic_edge, gic_intid};
use crate::arch::fdt::node::NodeRef;
use crate::arch::fdt::parser::Fdt;

/// The SPI the SMMU raises for `name` in its `interrupt-names`, or zero
/// when the tree wires none. One that is named but is not a rising-edge
/// SPI (the SMMUv3's wired interrupts are edge pulses) refuses the node,
/// rather than routing a line the unit never pulses.
pub(super) fn named_spi(fdt: &Fdt, node: &NodeRef, name: &[u8]) -> Result<u32, FdtError> {
    let Some(names) = fdt.property(node, b"interrupt-names")? else {
        return Ok(0);
    };
    let Some(index) = names.stringlist().position(|n| n == name) else {
        return Ok(0);
    };
    let spec = fdt.interrupt(node, index)?.ok_or(FdtError::BadCells)?;
    let intid = gic_intid(spec.cells)?;
    if intid < 32 || !gic_edge(spec.cells)? {
        return Err(FdtError::BadCells);
    }
    Ok(intid)
}
