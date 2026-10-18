// NONOS Operating System (AGPL-3.0-or-later)
//! Interrupt specifiers: the right controller, the right cell count, and GIC
//! numbers held to the ranges the binding allows.

use crate::arch::fdt::irq::{gic_edge, gic_intid};
use crate::arch::fdt::{Fdt, FdtError};
use crate::builder::{cells, Builder, END};
use crate::fixtures::RPI4B;

#[test]
fn spis_decode_against_the_gic_400() {
    let f = Fdt::from_bytes(RPI4B).unwrap();
    let intid = |p: &[u8], i| {
        let node = f.find_path(p).unwrap().unwrap();
        gic_intid(f.interrupt(&node, i).unwrap().unwrap().cells)
    };
    assert_eq!(intid(b"/soc/serial@7e215040", 0), Ok(125));
    assert_eq!(intid(b"/emmc2bus/mmc@7e340000", 0), Ok(158));
    assert_eq!(intid(b"/scb/ethernet@7d580000", 1), Ok(190));
}

#[test]
fn gic_numbers_outside_their_type_are_refused() {
    assert_eq!(gic_intid(&cells(&[1, 15, 4])), Ok(31));
    assert_eq!(gic_intid(&cells(&[1, 16, 4])), Err(FdtError::BadCells));
    assert_eq!(gic_intid(&cells(&[0, 987, 4])), Ok(1019));
    assert_eq!(gic_intid(&cells(&[0, 988, 4])), Err(FdtError::BadCells));
    assert_eq!(gic_intid(&cells(&[2, 0, 4])), Err(FdtError::BadCells));
    assert_eq!(gic_intid(&cells(&[0, 1])), Err(FdtError::BadCells));
    assert_eq!(gic_edge(&cells(&[0, 1, 8])), Ok(false));
}

fn tree(controller_cells: &[u8], interrupts: &[u8]) -> Vec<u8> {
    Builder::default()
        .begin("")
        .prop("interrupt-parent", &cells(&[1]))
        .begin("intc")
        .prop("phandle", &cells(&[1]))
        .prop("#interrupt-cells", controller_cells)
        .end()
        .begin("dev")
        .prop("interrupts", interrupts)
        .end()
        .end()
        .token(END)
        .finish()
}

fn first(blob: &[u8]) -> Result<Option<Vec<u8>>, FdtError> {
    let f = Fdt::from_bytes(blob).unwrap();
    let dev = f.find_path(b"/dev").unwrap().unwrap();
    f.interrupt(&dev, 0).map(|s| s.map(|s| s.cells.to_vec()))
}

#[test]
fn controller_cell_counts_are_checked() {
    assert_eq!(first(&tree(&cells(&[2]), &cells(&[5, 6]))), Ok(Some(cells(&[5, 6]))));
    assert_eq!(first(&tree(&cells(&[0]), &cells(&[5]))), Err(FdtError::BadCells));
    assert_eq!(first(&tree(&cells(&[9]), &cells(&[5]))), Err(FdtError::BadCells));
    assert_eq!(first(&tree(&cells(&[2]), &cells(&[5, 6, 7]))), Err(FdtError::BadCells));
}
