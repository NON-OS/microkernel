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

//! Every DesignWare I2C controller the device tree enables, for the broker
//! to list as device-tree bus records.

use crate::arch::fdt::error::FdtError;
use crate::arch::fdt::parser::Fdt;

use super::controllers::controllers;
use super::node::{DwI2c, IrqCtrl};
use super::phandle::provider;

/// Nesting followed; a controller deeper than this is not found.
pub const MAX_DEPTH: usize = 16;

/// Fills `out` and returns how many controllers the tree has, which may be
/// more than `out` holds. An interrupt is kept only when its parent names
/// a width of one to three cells and the node gave that many; the parent's
/// kind is noted either way.
pub fn find(fdt: &Fdt, out: &mut [DwI2c]) -> Result<usize, FdtError> {
    let found = controllers(fdt, out)?;
    for c in out.iter_mut().take(found) {
        let given = u32::from(c.irq_cells);
        let parent = match c.irq_parent {
            Some(h) => provider(fdt, h)?,
            None => None,
        };
        c.irq_ctrl = parent.map_or(IrqCtrl::Unknown, |p| p.ctrl);
        let width = parent.and_then(|p| p.interrupt_cells);
        let n = width.filter(|n| (1..=given).contains(n)).unwrap_or(0) as usize;
        c.irq_cells = n as u8;
        c.irq[n..].fill(0);
        if let Some(h) = c.clock {
            c.input_hz = provider(fdt, h)?.and_then(|p| p.fixed_hz);
        }
    }
    Ok(found)
}
