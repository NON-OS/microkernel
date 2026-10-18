// NONOS Operating System (AGPL-3.0-or-later)
//! INTx maps built to break each rule the parser holds.

use crate::arch::fdt::find::pci::{interrupt_map, IntxMap};
use crate::arch::fdt::{Fdt, FdtError};
use crate::builder::{cells, text, Builder, END};

/// An ECAM host whose map is `entries` (each: hi, pin, parent unit cells,
/// spec) beside a controller at phandle 1.
fn tree(compat: &str, mask: &[u32], entries: &[u32]) -> Vec<u8> {
    let mut b = Builder::default().begin("");
    b = b.prop("#address-cells", &cells(&[1])).prop("#size-cells", &cells(&[1]));
    b = b.begin("intc").prop("compatible", &text(compat)).prop("phandle", &cells(&[1]));
    b = b.prop("#interrupt-cells", &cells(&[3])).prop("#address-cells", &cells(&[0])).end();
    b = b.begin("pcie").prop("compatible", &text("pci-host-ecam-generic"));
    b = b.prop("reg", &cells(&[0x4000_0000, 0x1000_0000])).prop("#interrupt-cells", &cells(&[1]));
    b = b.prop("interrupt-map-mask", &cells(mask)).prop("interrupt-map", &cells(entries));
    b.end().end().token(END).finish()
}

fn parse(blob: &[u8]) -> Result<Option<IntxMap>, FdtError> {
    interrupt_map(&Fdt::from_bytes(blob).unwrap())
}

const MASK: [u32; 4] = [0x1800, 0, 0, 7];

#[test]
fn a_well_formed_built_map_routes_edge_and_level() {
    let m =
        parse(&tree("arm,gic-400", &MASK, &[0, 0, 0, 1, 1, 0, 9, 1, 0x800, 0, 0, 2, 1, 0, 10, 4]));
    let m = m.unwrap().unwrap();
    assert_eq!((m.route(0, 0, 0, 1), m.route(0, 1, 0, 2)), (Some((41, true)), Some((42, false))));
}

#[test]
fn every_untrustworthy_map_is_refused() {
    let bad = |blob: Vec<u8>, why: FdtError| assert_eq!(parse(&blob), Err(why));
    let entry = |flags: u32| [0, 0, 0, 1, 1, 0, 9, flags];
    bad(tree("arm,gic-400", &MASK, &entry(8)), FdtError::BadCells);
    bad(tree("arm,gic-400", &MASK, &entry(2)), FdtError::BadCells);
    bad(tree("arm,gic-400", &MASK, &[0, 0, 0, 1, 1, 1, 9, 4]), FdtError::BadCells);
    bad(tree("vendor,other-intc", &MASK, &entry(4)), FdtError::BadCells);
    bad(tree("arm,gic-400", &[0x1800, 1, 0, 7], &entry(4)), FdtError::BadCells);
    bad(tree("arm,gic-400", &MASK, &entry(4)[..7]), FdtError::OutOfBounds);
    bad(tree("arm,gic-400", &MASK, &[0, 0, 0, 1, 2, 0, 9, 4]), FdtError::NoSuchNode);
    let many: Vec<u32> = (0..65).flat_map(|n| [n << 11, 0, 0, 1, 1, 0, 9, 4]).collect();
    bad(tree("arm,gic-400", &MASK, &many), FdtError::BadCells);
}
