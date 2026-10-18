// NONOS Operating System (AGPL-3.0-or-later)
//! The generic ECAM host's `bus-range`: two bus numbers, in order.

use crate::arch::fdt::find::pci;
use crate::arch::fdt::{Fdt, FdtError};
use crate::builder::{cells, text, Builder, END};

fn host(bus_range: Option<&[u32]>) -> Vec<u8> {
    let mut b = Builder::default()
        .begin("")
        .begin("pcie@10000000")
        .prop("compatible", &text("pci-host-ecam-generic"))
        .prop("reg", &cells(&[0x40, 0x1000_0000, 0, 0x1000_0000]));
    if let Some(range) = bus_range {
        b = b.prop("bus-range", &cells(range));
    }
    b.end().end().token(END).finish()
}

fn buses(bus_range: Option<&[u32]>) -> Result<(u8, u8), FdtError> {
    let blob = host(bus_range);
    let f = Fdt::from_bytes(&blob).unwrap();
    pci::find(&f).map(|h| {
        let h = h.unwrap();
        (h.bus_start, h.bus_end)
    })
}

#[test]
fn bus_range_defaults_and_bounds() {
    assert_eq!(buses(None), Ok((0, 255)));
    assert_eq!(buses(Some(&[0x10, 0x1f])), Ok((0x10, 0x1f)));
    assert_eq!(buses(Some(&[0x20, 0x1f])), Err(FdtError::BadCells));
    assert_eq!(buses(Some(&[0, 256])), Err(FdtError::BadCells));
    assert_eq!(buses(Some(&[0])), Err(FdtError::BadCells));
}
