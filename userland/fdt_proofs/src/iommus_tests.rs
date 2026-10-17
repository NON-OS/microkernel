// NONOS Operating System (AGPL-3.0-or-later)
//! A device's `iommus`: its unit and streams when each entry is one cell on
//! one unit; what the kernel cannot honour is `Unreadable`, never cut
//! short; and a platform device carries what its node says.

use crate::arch::fdt::find::iommus::{iommus, Iommus};
use crate::arch::fdt::find::platform::{find, PlatformDevice};
use crate::arch::fdt::{Fdt, FdtError};
use crate::builder::{cells, text, Builder, END};

/// Units at phandles 1 and 3 (one cell) and 2 (two cells), and a device.
fn tree(prop: Option<&[u32]>) -> Vec<u8> {
    let mut b = Builder::default().begin("");
    b = b.prop("#address-cells", &cells(&[1])).prop("#size-cells", &cells(&[1]));
    b = b.begin("smmu").prop("#iommu-cells", &cells(&[1])).prop("phandle", &cells(&[1])).end();
    b = b.begin("other").prop("#iommu-cells", &cells(&[2])).prop("phandle", &cells(&[2])).end();
    b = b.begin("third").prop("#iommu-cells", &cells(&[1])).prop("phandle", &cells(&[3])).end();
    b = b
        .begin("dev@1000")
        .prop("compatible", &text("generic-xhci"))
        .prop("reg", &cells(&[0x1000, 0x100]));
    if let Some(prop) = prop {
        b = b.prop("iommus", &cells(prop));
    }
    b.end().end().token(END).finish()
}

fn read(prop: Option<&[u32]>) -> Result<Iommus, FdtError> {
    let blob = tree(prop);
    let fdt = Fdt::from_bytes(&blob).unwrap();
    let node = fdt.find_path(b"/dev@1000").unwrap().unwrap();
    iommus(&fdt, &node)
}

#[test]
fn streams_on_one_unit() {
    assert_eq!(read(None), Ok(Iommus::None));
    let two = Iommus::Streams { unit: 1, sids: [0x10, 0x11, 0, 0], count: 2 };
    assert_eq!(read(Some(&[1, 0x10, 1, 0x11])), Ok(two));
    let four = Iommus::Streams { unit: 1, sids: [1, 2, 3, 4], count: 4 };
    assert_eq!(read(Some(&[1, 1, 1, 2, 1, 3, 1, 4])), Ok(four));
}

#[test]
fn what_cannot_be_honoured_is_unreadable() {
    let one_cell: &[u8] = b"iommus entries that are not one cell each";
    assert_eq!(read(Some(&[])), Ok(Iommus::Unreadable(one_cell)));
    assert_eq!(read(Some(&[1, 0x10, 1])), Ok(Iommus::Unreadable(one_cell)));
    assert_eq!(read(Some(&[2, 0x10])), Ok(Iommus::Unreadable(one_cell)));
    let two_units: &[u8] = b"iommus naming more than one IOMMU";
    assert_eq!(read(Some(&[1, 0x10, 3, 0x11])), Ok(Iommus::Unreadable(two_units)));
    let mixed = read(Some(&[1, 0x10, 1, 0x11, 1, 0x12, 1, 0x13, 1, 0x14]));
    assert_eq!(mixed, Ok(Iommus::Unreadable(b"more stream ids than kept")));
    assert_eq!(read(Some(&[9, 0x10])), Err(FdtError::NoSuchNode));
}

#[test]
fn a_platform_device_carries_its_iommus() {
    for (prop, want) in [
        (None, Iommus::None),
        (Some(&[1u32, 0x20][..]), Iommus::Streams { unit: 1, sids: [0x20, 0, 0, 0], count: 1 }),
        (Some(&[1, 0x20, 3, 0x21][..]), Iommus::Unreadable(b"iommus naming more than one IOMMU")),
    ] {
        let blob = tree(prop);
        let mut out = [PlatformDevice::EMPTY; 2];
        assert_eq!(find(&Fdt::from_bytes(&blob).unwrap(), &mut out), Ok(1));
        assert_eq!(out[0].iommus, want);
    }
}
