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
//! class of the PCI function the same capsule serves, the reg window a
//! memory BAR, and the interrupt in `irq_source` (line.rs): the mapping
//! C's platform finder takes for it, reviewed against C's at integration.

use crate::arch::fdt::find::i2c::DwI2c;
use crate::hardware::broker::class::ids;
use crate::hardware::broker::device::{Bar, BarKind, DeviceRecord};

use super::line::line;

/// C's BusKind::Fdt.
pub(super) const BUS_KIND_FDT: u8 = 4;
/// The register interface: C's described::Interface numbers Sdhci 1, Xhci
/// 2 and Ahci 3; DesignWare I2C takes the next, a named entry in C's table
/// at integration.
pub(super) const INTERFACE_DW_I2C: u16 = 4;
/// Serial bus, as the ACPI controller's record says, so the capsule that
/// serves the PCI function recognises it; an arm matches this, the class,
/// the bus kind and the interface together.
pub(super) const PCI_CLASS_SERIAL_BUS: u8 = 0x0c;

/// The record for `c` on a riscv64 kernel or not, and whether its line is
/// edge-triggered: DEVICE_FLAG_IRQ_EDGE is F's, set from this once it is in
/// the base.
///
/// `bars[0].aux` is the DesignWare input clock (ic_clk) in Hz, from a fixed
/// clock only, else 0. `bars[1]` is no window: its aux carries the bus rate
/// the tree asks for (`clock-frequency`), as the I2C-HID record carries
/// data in a window-less BAR; `bar_count` stays 1. The line, when there is
/// one, is `irq_source` alone: `irq_line` is a PCI byte and stays 0xFF.
pub(super) fn device_record(c: &DwI2c, riscv: bool) -> (DeviceRecord, bool) {
    let mut bars = [Bar::empty(); 6];
    bars[0] = Bar {
        base: c.base,
        size: c.size,
        kind: BarKind::Mmio as u8,
        flags: 0,
        aux: c.input_hz.unwrap_or(0),
        _pad: [0; 2],
    };
    bars[1] = Bar { aux: c.bus_hz.unwrap_or(0), ..Bar::empty() };
    let found = line(c, riscv);
    let record = DeviceRecord {
        bus_kind: BUS_KIND_FDT,
        pci_class: PCI_CLASS_SERIAL_BUS,
        class: ids::SERIAL,
        device: INTERFACE_DW_I2C,
        bar_count: 1,
        irq_line: 0xFF,
        irq_pin: u8::from(found.is_some()),
        irq_source: found.map_or(0, |l| l.source),
        bars,
        ..DeviceRecord::empty()
    };
    (record, found.is_some_and(|l| l.edge))
}
