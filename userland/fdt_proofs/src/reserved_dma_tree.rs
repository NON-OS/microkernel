// NONOS Operating System (AGPL-3.0-or-later)
//! A tree with reserved DMA ranges, built to order: a GIC, an SMMU with one
//! stream cell, an ECAM host bridge with one function under it, a platform
//! device, and `/reserved-memory` children whose `iommu-addresses` name them.

use crate::arch::fdt::find::reserved_dma::{ranges, Refusal, TreeRange, MAX_TREE_RANGES};
use crate::arch::fdt::Fdt;
use crate::builder::{cells, text, Builder, END};

pub const SMMU: u32 = 2;
pub const BRIDGE: u32 = 3;
pub const FUNCTION: u32 = 4;
pub const PLATFORM: u32 = 5;
pub const BARE: u32 = 6;

/// One `/reserved-memory` child: its `reg` (base, size), if any, and its
/// raw `iommu-addresses`. Child `i` is phandle `100 + i`.
pub type Child = (Option<(u32, u32)>, Vec<u32>);

/// Every device's `memory-region`: each child but those in `unnamed`.
fn regions(children: &[Child], unnamed: &[usize]) -> Vec<u8> {
    let named = (0..children.len()).filter(|i| !unnamed.contains(i)).map(|i| 100 + i as u32);
    cells(&named.collect::<Vec<_>>())
}

pub fn tree(children: &[Child], domain: Option<u32>) -> Vec<u8> {
    tree_naming(children, domain, &[])
}

/// As `tree`, with the children in `unnamed` named back by no device.
pub fn tree_naming(children: &[Child], domain: Option<u32>, unnamed: &[usize]) -> Vec<u8> {
    let back = regions(children, unnamed);
    let mut b = Builder::default().begin("").prop("interrupt-parent", &cells(&[1]));
    b = b.prop("#address-cells", &cells(&[2])).prop("#size-cells", &cells(&[2]));
    b = b.begin("smmu").prop("#iommu-cells", &cells(&[1])).prop("phandle", &cells(&[SMMU])).end();
    b = b.begin("pcie").prop("device_type", &text("pci")).prop("phandle", &cells(&[BRIDGE]));
    b = b.prop("bus-range", &cells(&[0, 0x3f]));
    if let Some(domain) = domain {
        b = b.prop("linux,pci-domain", &cells(&[domain]));
    }
    b = b.prop("#address-cells", &cells(&[3])).prop("#size-cells", &cells(&[2]));
    b = b.prop("memory-region", &back);
    // Bus 1, device 2, function 3: phys.hi 0x0001_1300.
    b = b.begin("ethernet@0").prop("reg", &cells(&[0x0001_1300, 0, 0, 0, 0]));
    b = b.prop("phandle", &cells(&[FUNCTION])).prop("memory-region", &back).end().end();
    b = b.begin("display").prop("phandle", &cells(&[PLATFORM]));
    b = b.prop("iommus", &cells(&[SMMU, 0x10, SMMU, 0x11])).prop("memory-region", &back).end();
    b = b.begin("bare").prop("phandle", &cells(&[BARE])).prop("memory-region", &back).end();
    b = b.begin("reserved-memory").prop("#address-cells", &cells(&[2]));
    b = b.prop("#size-cells", &cells(&[2])).prop("ranges", &[]);
    for (i, (reg, addresses)) in children.iter().enumerate() {
        b = b.begin(&format!("r{i}")).prop("phandle", &cells(&[100 + i as u32]));
        if let Some((base, size)) = reg {
            b = b.prop("reg", &cells(&[0, *base, 0, *size]));
        }
        b = b.prop("iommu-addresses", &cells(addresses)).end();
    }
    b.end().end().token(END).finish()
}

/// What boot keeps: the entries, and the first unkeyed reason.
pub fn read(blob: &[u8]) -> Result<(Vec<TreeRange>, Option<&'static [u8]>), Refusal> {
    let fdt = Fdt::from_bytes(blob).unwrap();
    let (mut out, mut unkeyed) = ([TreeRange::EMPTY; MAX_TREE_RANGES], None);
    let n = ranges(&fdt, &mut out, &mut unkeyed)?;
    Ok((out[..n].to_vec(), unkeyed))
}
