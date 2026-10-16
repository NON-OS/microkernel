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

//! A node's properties noted as the walk meets them, and the controller a
//! finished node makes.

use crate::arch::fdt::endian::be_u32;
use crate::arch::fdt::error::FdtError;
use crate::arch::fdt::property::Property;

use super::level::Level;
use super::node::DwI2c;

const COMPATIBLE: &[u8] = b"snps,designware-i2c";

/// `parent` is the cells the parent declared, which this node's `reg` uses.
pub(super) fn note(l: &mut Level, p: &Property, parent: (u32, u32)) -> Result<(), FdtError> {
    match p.name {
        b"#address-cells" => l.cells.0 = p.u32()?,
        b"#size-cells" => l.cells.1 = p.u32()?,
        b"ranges" => l.ranges_empty = p.data.is_empty(),
        b"interrupt-parent" => l.irq_parent = Some(p.u32()?),
        b"compatible" => l.dw = p.compatible_matches(COMPATIBLE),
        b"status" => l.okay = matches!(p.stringlist().next(), Some(b"okay" | b"ok")),
        b"reg" => l.reg = p.reg_iter(parent.0, parent.1).next(),
        b"interrupts" => l.irq = cells3(p),
        b"clocks" => l.clock = be_u32(p.data, 0).ok(),
        b"clock-frequency" => l.bus_hz = be_u32(p.data, 0).ok(),
        _ => {}
    }
    Ok(())
}

/// The controller a node is, once all its properties are seen. The
/// interrupt cells are the count `interrupts` held until the second pass.
pub(super) fn controller(l: &Level) -> Option<DwI2c> {
    let (base, size) = l.reg.filter(|_| l.dw && l.okay && l.reaches)?;
    Some(DwI2c {
        base,
        size,
        irq: l.irq.0,
        irq_cells: l.irq.1,
        irq_parent: l.irq_parent_in_force(),
        bus_hz: l.bus_hz,
        clock: l.clock,
        input_hz: None,
    })
}

/// Up to three cells of a property, and how many it held.
fn cells3(p: &Property) -> ([u32; 3], u8) {
    let mut out = [0u32; 3];
    let n = (p.data.len() / 4).min(3);
    for (i, c) in out.iter_mut().enumerate().take(n) {
        *c = be_u32(p.data, i * 4).unwrap_or(0);
    }
    (out, n as u8)
}
