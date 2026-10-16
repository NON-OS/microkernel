// NONOS Operating System (AGPL-3.0-or-later)
//! A device-tree DesignWare I2C controller as the broker lists it: bus
//! kind 4, its window a memory BAR carrying the input clock, and its
//! interrupt only when the specifier's shape says which line it is.

use super::device::BarKind;
use super::fdt_i2c_record::{device_record, INTERFACE_DW_I2C};
use crate::arch::fdt::find::i2c::DwI2c;
use crate::arch::fdt::i2c_tests::{run, soc};

fn found() -> [DwI2c; 4] {
    let mut out = [DwI2c::default(); 4];
    run(&soc(), &mut out);
    out
}

#[test]
fn a_gic_spi_controller_is_a_bus_kind_four_serial_record() {
    let r = device_record(&found()[0]);
    assert_eq!((r.bus_kind, r.vendor, r.device), (4, 0, INTERFACE_DW_I2C));
    assert_eq!((r.class, r.pci_class, r.bar_count), (0x0060, 0x0c, 1));
    let b = r.bars[0];
    assert_eq!((b.base, b.size, b.kind), (0x1000, 0x100, BarKind::Mmio as u8));
    assert_eq!(b.aux, 100_000_000, "the input clock, not the 400 kHz bus rate");
    assert_eq!((r.irq_source, r.irq_line, r.irq_pin), (32, 32, 1));
    assert_eq!((r.flags, r.device_id), (0, 0), "the registry assigns the id");
}

#[test]
fn an_interrupt_whose_parent_is_missing_is_no_line() {
    let r = device_record(&found()[1]);
    assert_eq!((r.bars[0].base, r.bars[0].aux), (0x3000, 0), "no clock found");
    assert_eq!((r.irq_source, r.irq_line, r.irq_pin), (0, 0xFF, 0));
}

#[test]
fn only_a_spi_or_a_single_cell_names_the_line() {
    let at = |cells, irq| DwI2c { irq_cells: cells, irq, ..found()[0] };
    let line = |c: DwI2c| {
        let r = device_record(&c);
        (r.irq_source, r.irq_line, r.irq_pin)
    };
    assert_eq!(line(at(1, [7, 0, 0])), (7, 7, 1));
    assert_eq!(line(at(1, [300, 0, 0])), (300, 0xFF, 1), "past the legacy byte");
    assert_eq!(line(at(3, [1, 29, 4])), (0, 0xFF, 0), "a PPI is per core, not a device line");
    assert_eq!(line(at(2, [5, 1, 0])), (0, 0xFF, 0), "two cells: not a shape we know");
}
