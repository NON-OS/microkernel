// NONOS Operating System (AGPL-3.0-or-later)
//! Host bridges and MSI controllers built to order for the msi tests.

use crate::arch::fdt::find::msi::{pci_msi_map, MsiKind};
use crate::arch::fdt::{Fdt, FdtError};
use crate::builder::{cells, text, Builder, END};

/// Properties, each a name and its cells.
pub type Props<'a> = &'a [(&'a str, &'a [u32])];
/// A controller: compatible, `reg` size, and its other properties.
pub type Unit<'a> = (&'a str, u32, Props<'a>);

/// A host bridge carrying `host` (name, cells) beside controllers at
/// phandles 1 and 2, each `(compatible, reg size, extra props)`.
pub fn tree(host: Props, units: &[Unit]) -> Vec<u8> {
    let mut b = Builder::default().begin("");
    b = b.prop("#address-cells", &cells(&[1])).prop("#size-cells", &cells(&[1]));
    for (n, (compat, size, props)) in units.iter().enumerate() {
        b = b.begin(&format!("msi{n}")).prop("compatible", &text(compat));
        b = b.prop("reg", &cells(&[0x10_0000 * (n as u32 + 1), *size]));
        b = b.prop("phandle", &cells(&[n as u32 + 1])).prop("msi-controller", &[]);
        for (name, value) in props.iter() {
            b = b.prop(name, &cells(value));
        }
        b = b.end();
    }
    b = b.begin("pcie").prop("compatible", &text("pci-host-ecam-generic"));
    b = b.prop("reg", &cells(&[0x4000_0000, 0x1000_0000]));
    for (name, value) in host {
        b = b.prop(name, &cells(value));
    }
    b.end().end().token(END).finish()
}

pub const ITS: Unit<'static> = ("arm,gic-v3-its", 0x2_0000, &[("#msi-cells", &[1])]);
pub const V2M: Unit<'static> = ("arm,gic-v2m-frame", 0x1000, &[]);

pub fn kind(blob: &[u8]) -> Result<Option<(MsiKind, u32)>, FdtError> {
    let found = pci_msi_map(&Fdt::from_bytes(blob).unwrap())?;
    Ok(found.map(|(c, map)| (c.kind, map.output(0x0105, c.phandle).unwrap_or(u32::MAX))))
}
