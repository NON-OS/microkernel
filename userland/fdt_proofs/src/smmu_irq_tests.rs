// NONOS Operating System (AGPL-3.0-or-later)
//! The SMMU's event queue and global error interrupts, by name.

use crate::arch::fdt::find::smmu::find;
use crate::arch::fdt::{Fdt, FdtError};
use crate::builder::{cells, text, Builder, END};

/// An SMMU whose interrupts are `specs` (three cells each) named `names`.
fn tree(names: &[&str], specs: &[u32]) -> Vec<u8> {
    let joined: Vec<u8> = names.iter().flat_map(|n| text(n)).collect();
    let mut b = Builder::default().begin("");
    b = b.prop("#address-cells", &cells(&[1])).prop("#size-cells", &cells(&[1]));
    b = b.begin("intc").prop("compatible", &text("arm,gic-v3")).prop("phandle", &cells(&[2]));
    b = b.prop("#interrupt-cells", &cells(&[3])).end();
    b = b.begin("smmu").prop("compatible", &text("arm,smmu-v3"));
    b = b.prop("reg", &cells(&[0x10_0000, 0x2_0000])).prop("interrupt-parent", &cells(&[2]));
    b = b.prop("interrupts", &cells(specs)).prop("interrupt-names", &joined).end();
    b.end().token(END).finish()
}

fn intids(blob: &[u8]) -> Result<(u32, u32), FdtError> {
    let info = find(&Fdt::from_bytes(blob).unwrap())?.unwrap();
    Ok((info.eventq_intid, info.gerror_intid))
}

#[test]
fn named_edge_spis_are_found_in_any_order() {
    let blob = tree(&["gerror", "priq", "eventq"], &[0, 77, 1, 0, 75, 1, 0, 74, 1]);
    assert_eq!(intids(&blob), Ok((106, 109)));
    assert_eq!(intids(&tree(&["priq"], &[0, 75, 1])), Ok((0, 0)), "neither wired");
}

#[test]
fn a_named_interrupt_that_is_not_an_edge_spi_is_refused() {
    let level = tree(&["eventq"], &[0, 74, 4]);
    let ppi = tree(&["eventq"], &[1, 9, 1]);
    let missing = tree(&["priq", "eventq"], &[0, 75, 1]);
    for blob in [level, ppi, missing] {
        assert_eq!(intids(&blob), Err(FdtError::BadCells));
    }
}
