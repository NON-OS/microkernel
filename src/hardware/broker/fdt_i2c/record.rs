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

//! A DesignWare I2C controller found in the device tree (arch/fdt/find/i2c)
//! as the broker lists it, in C's device-tree record shape: bus kind 4
//! (C's BusKind::Fdt), vendor 0, the register interface in `device`, the
//! class of the PCI function the same capsule serves, each `reg` window a
//! memory BAR, the interrupt in `irq_source`, and the input clock in
//! `bars[0].aux` as an ACPI controller carries it (acpi_i2c/record.rs). C
//! registers records from its platform finder; this is the mapping that
//! path takes for this controller, reviewed against C's at integration.

use crate::arch::fdt::find::i2c::DwI2c;
use crate::hardware::broker::class::ids;
use crate::hardware::broker::device::{Bar, BarKind, DeviceRecord};

/// C's BusKind::Fdt.
const BUS_KIND_FDT: u8 = 4;
/// The register interface: C's described::Interface numbers Sdhci 1, Xhci
/// 2 and Ahci 3; DesignWare I2C takes the next, for C to confirm.
pub(super) const INTERFACE_DW_I2C: u16 = 4;
/// Serial bus, as the ACPI controller's record says, so the capsule that
/// serves the PCI function recognises it.
const PCI_CLASS_SERIAL_BUS: u8 = 0x0c;
/// A GIC interrupt specifier is (type, number, flags), type 0 an SPI.
const GIC_SPI: u32 = 0;

/// The record for `c`. The interrupt is the GIC SPI number of a three-cell
/// specifier, or the one cell of a one-cell one; any other is no line.
pub(super) fn device_record(c: &DwI2c) -> DeviceRecord {
    let mut bars = [Bar::empty(); 6];
    bars[0] = Bar {
        base: c.base,
        size: c.size,
        kind: BarKind::Mmio as u8,
        flags: 0,
        aux: c.input_hz.unwrap_or(0),
        _pad: [0; 2],
    };
    let irq = match (c.irq_cells, c.irq) {
        (3, [GIC_SPI, n, _]) => Some(n),
        (1, [n, _, _]) => Some(n),
        _ => None,
    };
    DeviceRecord {
        bus_kind: BUS_KIND_FDT,
        pci_class: PCI_CLASS_SERIAL_BUS,
        class: ids::SERIAL,
        device: INTERFACE_DW_I2C,
        bar_count: 1,
        irq_line: irq.filter(|&n| n < 0xFF).map_or(0xFF, |n| n as u8),
        irq_pin: u8::from(irq.is_some()),
        irq_source: irq.unwrap_or(0),
        bars,
        ..DeviceRecord::empty()
    }
}
