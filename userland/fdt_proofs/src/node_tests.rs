// NONOS Operating System (AGPL-3.0-or-later)
//! Paths, aliases, phandles and cell counts.

use crate::arch::fdt::{Fdt, FdtError};
use crate::builder::{cells, text, Builder, END};
use crate::fixtures::{RPI4B, VIRT_GICV3};

#[test]
fn paths_match_with_or_without_unit_addresses() {
    let f = Fdt::from_bytes(RPI4B).unwrap();
    let name = |p: &[u8]| f.find_path(p).unwrap().map(|n| n.name);
    assert_eq!(name(b"/"), Some(&b""[..]));
    assert_eq!(name(b"/soc/serial"), Some(&b"serial@7e201000"[..]));
    assert_eq!(name(b"/soc/serial@7e215040/"), Some(&b"serial@7e215040"[..]));
    assert_eq!(name(b"/soc/serial@7e215041"), None);
    assert_eq!(name(b"soc"), None);
    assert_eq!(name(&b"/a".repeat(40)), None);
}

#[test]
fn aliases_resolve_with_and_without_a_tail() {
    let f = Fdt::from_bytes(RPI4B).unwrap();
    let name = |p: &[u8]| f.resolve(p).unwrap().map(|n| n.name);
    assert_eq!(name(b"uart0"), Some(&b"serial@7e201000"[..]));
    assert_eq!(name(b"serial1/bluetooth"), Some(&b"bluetooth"[..]));
    assert_eq!(name(b"nonesuch"), None);
    let bt = f.resolve(b"serial0/bluetooth").unwrap().unwrap();
    assert_eq!(f.is_enabled(&bt), Ok(false));
    assert_eq!(f.chain(&bt).unwrap().as_slice().len(), 4);
}

#[test]
fn phandles_and_children() {
    let f = Fdt::from_bytes(VIRT_GICV3).unwrap();
    assert_eq!(f.by_phandle(0x8003).unwrap().name, b"intc@8000000");
    assert_eq!(f.by_phandle(0).err(), Some(FdtError::NoSuchNode));
    assert_eq!(f.by_phandle(0x1234).err(), Some(FdtError::NoSuchNode));
    let cpus = f.find_path(b"/cpus").unwrap().unwrap();
    let names: Vec<_> = f.children(&cpus).unwrap().map(|c| c.unwrap().name).collect();
    assert_eq!(names, [&b"cpu-map"[..], b"cpu@0", b"cpu@1"]);
    assert_eq!(f.own_cells(&cpus).unwrap().address, 1);
}

fn with_cells(address: &[u8]) -> Vec<u8> {
    Builder::default()
        .begin("")
        .prop("#address-cells", address)
        .begin("dev")
        .prop("reg", &cells(&[1, 2, 3, 4]))
        .end()
        .end()
        .token(END)
        .finish()
}

#[test]
fn hostile_cell_counts_cost_the_address_not_the_walk() {
    for bad in [cells(&[0]), cells(&[5]), cells(&[1, 1]), text("x")] {
        let blob = with_cells(&bad);
        let f = Fdt::from_bytes(&blob).unwrap();
        let dev = f.find_path(b"/dev").unwrap().unwrap();
        assert_eq!(f.mmio(&dev, 0), Err(FdtError::BadCells));
        assert_eq!(f.reg(&dev).unwrap().unwrap().count(), 0);
    }
    let blob = with_cells(&cells(&[1]));
    let f = Fdt::from_bytes(&blob).unwrap();
    let dev = f.find_path(b"/dev").unwrap().unwrap();
    assert_eq!(f.mmio(&dev, 0), Ok(Some((1, 2))));
    assert_eq!(f.mmio(&dev, 1), Ok(Some((3, 4))));
}
