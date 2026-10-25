// NONOS Operating System (AGPL-3.0-or-later)
//! `/chosen`'s `nonos,acpi-rsdp`, and which description the kernel boots
//! from: the tree whenever it names a GIC, ACPI only for a stub tree.

use crate::arch::fdt::chosen::Tables;
use crate::arch::fdt::{Fdt, FdtError};
use crate::builder::{cells, text, Builder, END};
use crate::fixtures::{VIRT_GICV2, VIRT_GICV3};

/// A tree with an optional GICv3 node and a `/chosen` holding `rsdp`.
fn tree(gic: bool, rsdp: Option<Vec<u8>>) -> Vec<u8> {
    let mut b = Builder::default().begin("");
    b = b.prop("#address-cells", &cells(&[2])).prop("#size-cells", &cells(&[2]));
    if gic {
        b = b.begin("interrupt-controller@8000000").prop("compatible", &text("arm,gic-v3"));
        b = b.prop("interrupt-controller", &[]).prop("#interrupt-cells", &cells(&[3]));
        let reg = [0, 0x0800_0000, 0, 0x1_0000, 0, 0x080A_0000, 0, 0xF6_0000];
        b = b.prop("reg", &cells(&reg)).end();
    }
    b = b.begin("chosen");
    if let Some(value) = rsdp {
        b = b.prop("nonos,acpi-rsdp", &value);
    }
    b.end().end().token(END).finish()
}

fn read(blob: &[u8]) -> (Result<Option<u64>, FdtError>, Result<Tables, FdtError>) {
    let fdt = Fdt::from_bytes(blob).unwrap();
    (fdt.acpi_rsdp(), fdt.tables())
}

#[test]
fn the_rsdp_is_one_or_two_cells_and_never_zero() {
    assert_eq!(read(&tree(false, Some(cells(&[0x7FF0_0000])))).0, Ok(Some(0x7FF0_0000)));
    assert_eq!(read(&tree(false, Some(cells(&[0x8, 0x10])))).0, Ok(Some(0x8_0000_0010)));
    assert_eq!(read(&tree(false, None)).0, Ok(None));
    assert_eq!(read(&tree(false, Some(cells(&[0])))).0, Err(FdtError::OutOfBounds));
    assert_eq!(read(&tree(false, Some(cells(&[0, 0])))).0, Err(FdtError::OutOfBounds));
    assert_eq!(read(&tree(false, Some(cells(&[1, 2, 3])))).0, Err(FdtError::BadCells));
    assert_eq!(read(&tree(false, Some(vec![1, 2]))).0, Err(FdtError::BadCells));
}

#[test]
fn a_stub_tree_naming_an_rsdp_boots_from_acpi() {
    assert_eq!(read(&tree(false, Some(cells(&[0x7FF0_0000])))).1, Ok(Tables::Acpi(0x7FF0_0000)));
}

/// A tree that describes the board wins, an RSDP or not; the RSDP is
/// still read, for the table readers that want it on a tree boot.
#[test]
fn a_tree_with_a_gic_stays_first() {
    let (rsdp, tables) = read(&tree(true, Some(cells(&[0x7FF0_0000]))));
    assert_eq!((rsdp, tables), (Ok(Some(0x7FF0_0000)), Ok(Tables::DeviceTree)));
    assert_eq!(read(&tree(true, None)).1, Ok(Tables::DeviceTree));
    for blob in [VIRT_GICV2, VIRT_GICV3] {
        assert_eq!(read(blob), (Ok(None), Ok(Tables::DeviceTree)));
    }
}

/// Neither a GIC nor an RSDP is still the tree's: the GIC step then
/// refuses it with its own reason, rather than this one guessing.
#[test]
fn neither_stays_with_the_tree() {
    assert_eq!(read(&tree(false, None)).1, Ok(Tables::DeviceTree));
}

/// A broken RSDP property breaks the decision too, instead of falling
/// back to the tree on a machine that has none.
#[test]
fn a_broken_rsdp_is_not_a_tree_boot() {
    assert_eq!(read(&tree(false, Some(cells(&[0])))).1, Err(FdtError::OutOfBounds));
}
