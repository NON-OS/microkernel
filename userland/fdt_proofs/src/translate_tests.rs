// NONOS Operating System (AGPL-3.0-or-later)
//! `ranges` translation refuses anything it cannot map whole.

use crate::arch::fdt::{Fdt, FdtError};
use crate::builder::{cells, Builder, END};
use crate::fixtures::RPI4B;

#[test]
fn a_bus_without_ranges_is_not_memory_mapped() {
    let f = Fdt::from_bytes(RPI4B).unwrap();
    let mdio = f.find_path(b"/scb/ethernet@7d580000/mdio@e14").unwrap().unwrap();
    assert_eq!(f.mmio(&mdio, 0), Err(FdtError::NotTranslatable));
}

fn bus(ranges: &[u32], reg: &[u32]) -> Vec<u8> {
    Builder::default()
        .begin("")
        .begin("bus")
        .prop("#address-cells", &cells(&[1]))
        .prop("#size-cells", &cells(&[1]))
        .prop("ranges", &cells(ranges))
        .begin("dev")
        .prop("reg", &cells(reg))
        .end()
        .end()
        .end()
        .token(END)
        .finish()
}

fn dev_mmio(blob: &[u8]) -> Result<Option<(u64, u64)>, FdtError> {
    let f = Fdt::from_bytes(blob).unwrap();
    let dev = f.find_path(b"/bus/dev").unwrap().unwrap();
    f.mmio(&dev, 0)
}

#[test]
fn a_span_has_to_fit_inside_one_window() {
    let window = [0x1000, 0, 0x8000_0000, 0x1000];
    assert_eq!(dev_mmio(&bus(&window, &[0x1800, 0x800])), Ok(Some((0x8000_0800, 0x800))));
    assert_eq!(dev_mmio(&bus(&window, &[0x1800, 0x801])), Err(FdtError::NotTranslatable));
    assert_eq!(dev_mmio(&bus(&window, &[0x0800, 0x10])), Err(FdtError::NotTranslatable));
    assert_eq!(dev_mmio(&bus(&window, &[0x2000, 0x10])), Err(FdtError::NotTranslatable));
}

#[test]
fn a_window_that_wraps_the_address_space_is_refused() {
    let wrap = [0x1000, 0xffff_ffff, 0xffff_f000, 0x2000];
    assert_eq!(dev_mmio(&bus(&wrap, &[0x1800, 0x10])), Err(FdtError::NotTranslatable));
    assert_eq!(dev_mmio(&bus(&[], &[0x1800, 0x10])), Ok(Some((0x1800, 0x10))));
}
