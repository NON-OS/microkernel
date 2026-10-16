// NONOS Operating System (AGPL-3.0-or-later)
//! A device-tree DesignWare I2C controller as the broker lists it: bus
//! kind 4, its window a memory BAR carrying the input clock, the bus rate
//! in a window-less second BAR, and an interrupt only when the parent is a
//! controller whose specifier is understood: a GIC SPI as its INTID, or a
//! PLIC source on riscv64.

use super::line::{line, Line};
use super::record::{device_record, BUS_KIND_FDT, INTERFACE_DW_I2C, PCI_CLASS_SERIAL_BUS};
use crate::arch::fdt::find::i2c::{DwI2c, IrqCtrl};
use crate::arch::fdt::i2c_tests::{run, soc};
use crate::hardware::broker::device::BarKind;

fn found() -> [DwI2c; 4] {
    let mut out = [DwI2c::default(); 4];
    run(&soc(), &mut out);
    out
}

#[test]
fn a_gic_spi_controller_is_a_bus_kind_four_serial_record() {
    let (r, edge) = device_record(&found()[0], false);
    assert_eq!((r.bus_kind, r.vendor, r.device), (BUS_KIND_FDT, 0, INTERFACE_DW_I2C));
    assert_eq!((r.class, r.pci_class, r.bar_count), (0x0060, PCI_CLASS_SERIAL_BUS, 1));
    let b = r.bars[0];
    assert_eq!((b.base, b.size, b.kind), (0x1000, 0x100, BarKind::Mmio as u8));
    assert_eq!(b.aux, 100_000_000, "the input clock, not the 400 kHz bus rate");
    let rate = r.bars[1];
    assert_eq!((rate.kind, rate.base, rate.size, rate.aux), (BarKind::None as u8, 0, 0, 400_000));
    assert_eq!((r.irq_source, r.irq_line, r.irq_pin, edge), (64, 0xFF, 1, false), "SPI 32");
    assert_eq!((r.flags, r.device_id), (0, 0), "the registry assigns the id");
}

#[test]
fn a_missing_parent_is_no_line_but_still_a_record() {
    assert_eq!(found()[1].irq_ctrl, IrqCtrl::Unknown, "no node has its parent's phandle");
    let (r, _) = device_record(&found()[1], false);
    assert_eq!((r.bars[0].base, r.bars[0].aux, r.bars[1].aux), (0x3000, 0, 0), "no clocks");
    assert_eq!((r.irq_source, r.irq_line, r.irq_pin), (0, 0xFF, 0));
}

#[test]
fn only_a_gic_spi_or_a_riscv_plic_source_is_a_line() {
    let at = |ctrl, cells, irq| DwI2c { irq_ctrl: ctrl, irq_cells: cells, irq, ..found()[0] };
    let gic = |irq| line(&at(IrqCtrl::Gic, 3, irq), false);
    assert_eq!(gic([0, 0, 4]), Some(Line { source: 32, edge: false }), "the first SPI");
    assert_eq!(gic([0, 987, 1]), Some(Line { source: 1019, edge: true }), "the last, rising");
    assert_eq!(gic([0, 988, 4]), None, "1020 and up are not SPIs");
    assert_eq!(gic([0, u32::MAX, 4]), None);
    assert_eq!(gic([1, 29, 4]), None, "a PPI is per core");
    assert_eq!(line(&at(IrqCtrl::Gic, 1, [7, 0, 0]), false), None, "one cell on a GIC");
    let plic = |riscv, n| line(&at(IrqCtrl::Plic, 1, [n, 0, 0]), riscv);
    assert_eq!(plic(true, 7), Some(Line { source: 7, edge: false }));
    assert_eq!((plic(false, 7), plic(true, 0)), (None, None), "aarch64; source 0");
    for ctrl in [IrqCtrl::Other, IrqCtrl::Unknown] {
        assert_eq!(line(&at(ctrl, 3, [0, 5, 4]), true), None, "{ctrl:?}");
    }
}
