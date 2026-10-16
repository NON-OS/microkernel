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

//! The interrupt a device-tree DesignWare I2C controller's record names,
//! decided by what its interrupt parent is (find/i2c's IrqCtrl), never by
//! the specifier's cell count alone:
//! - an Arm GIC, three cells (type, number, flags): type 0 is an SPI, INTID
//!   32 + number, and SPIs are INTIDs 32 to 1019; a PPI (type 1) is per
//!   core, never one device's line (GIC binding and architecture, as
//!   recalled, unverified);
//! - a RISC-V PLIC, one cell, on riscv64 only: the source number, 0 being
//!   no source;
//! - anything else: no line, rather than a guess.
//!
//! The tree's shared GIC decode (C's gic_intid) replaces this one at
//! integration; the rules are the same.

use crate::arch::fdt::find::i2c::{DwI2c, IrqCtrl};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Line {
    /// The GIC INTID or PLIC source, as `irq_source` carries it.
    pub source: u32,
    pub edge: bool,
}

const GIC_SPI: u32 = 0;
const SPI_FIRST: u32 = 32;
const SPI_LAST: u32 = 1019;
/// The flags cell's rising and falling edge bits (as recalled, unverified).
const GIC_EDGE: u32 = 0b11;

/// `riscv` is whether this kernel is riscv64, the only one a PLIC serves.
pub(super) fn line(c: &DwI2c, riscv: bool) -> Option<Line> {
    match (c.irq_ctrl, c.irq_cells, c.irq) {
        (IrqCtrl::Gic, 3, [GIC_SPI, n, flags]) => {
            let intid = SPI_FIRST.checked_add(n).filter(|i| *i <= SPI_LAST)?;
            Some(Line { source: intid, edge: flags & GIC_EDGE != 0 })
        }
        // A PLIC source is level-triggered (as recalled, unverified).
        (IrqCtrl::Plic, 1, [n, _, _]) if riscv && n != 0 => Some(Line { source: n, edge: false }),
        _ => None,
    }
}
