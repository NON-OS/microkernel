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

//! A DesignWare I2C controller as the device tree describes it, ready to
//! become a device-tree bus record.

/// What the interrupt parent is, by its compatible: an Arm GIC, a RISC-V
/// PLIC, another controller, or not known (no parent, or not found).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum IrqCtrl {
    #[default]
    Unknown,
    Gic,
    Plic,
    Other,
}

/// One `snps,designware-i2c` node whose status is okay and whose registers
/// sit at a CPU address.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct DwI2c {
    /// The first `reg` entry, translated to a CPU address.
    pub base: u64,
    pub size: u64,
    /// The first `interrupts` specifier, `irq_cells` cells of it; zero
    /// cells when the interrupt parent or its cell count is unknown.
    pub irq: [u32; 3],
    pub irq_cells: u8,
    /// The interrupt controller's phandle, the node's own or inherited,
    /// and what that controller is.
    pub irq_parent: Option<u32>,
    pub irq_ctrl: IrqCtrl,
    /// `clock-frequency`: the SCL rate asked of the bus, not the input.
    pub bus_hz: Option<u32>,
    /// The first `clocks` phandle, and that clock's rate when it is a
    /// fixed clock; else the input clock has to be found another way.
    pub clock: Option<u32>,
    pub input_hz: Option<u32>,
}
