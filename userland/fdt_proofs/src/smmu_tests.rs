// NONOS Operating System (AGPL-3.0-or-later)
//! The SMMUv3 finder and a host bridge's `iommu-map`: the virt tree, then
//! maps no board ships, built to pin each rule.

use crate::arch::fdt::find::smmu::{find, pci_iommu_map, SmmuInfo};
use crate::arch::fdt::{Fdt, FdtError};
use crate::builder::{cells, text, Builder, END};
use crate::fixtures::{VIRT_GICV3, VIRT_SMMUV3};

#[test]
fn virt_names_one_coherent_smmu_and_maps_every_rid_to_itself() {
    let f = Fdt::from_bytes(VIRT_SMMUV3).unwrap();
    let want = SmmuInfo { base: 0x0905_0000, size: 0x2_0000, phandle: 0x8005, coherent: true };
    assert_eq!(find(&f), Ok(Some(want)));
    let map = pci_iommu_map(&f).unwrap().unwrap();
    assert_eq!((map.count, map.mask), (1, u32::MAX));
    assert_eq!(map.stream_id(0x0008, 0x8005), Some(0x0008));
    assert_eq!(map.stream_id(0xffff, 0x8005), Some(0xffff));
    assert_eq!(map.stream_id(0x0008, 0x8004), None);
    let plain = Fdt::from_bytes(VIRT_GICV3).unwrap();
    assert_eq!((find(&plain), pci_iommu_map(&plain)), (Ok(None), Ok(None)));
}

/// An ECAM host with `map` (and `mask`, if any) beside an SMMU at phandle 1
/// taking `iommu_cells`.
fn tree(map: &[u32], mask: Option<u32>, iommu_cells: u32) -> Vec<u8> {
    let mut b = Builder::default().begin("");
    b = b.prop("#address-cells", &cells(&[1])).prop("#size-cells", &cells(&[1]));
    b = b.begin("smmu").prop("compatible", &text("arm,smmu-v3"));
    b = b.prop("reg", &cells(&[0x10_0000, 0x2_0000])).prop("phandle", &cells(&[1]));
    b = b.prop("#iommu-cells", &cells(&[iommu_cells])).end();
    b = b.begin("pcie").prop("compatible", &text("pci-host-ecam-generic"));
    b = b.prop("reg", &cells(&[0x4000_0000, 0x1000_0000])).prop("iommu-map", &cells(map));
    if let Some(mask) = mask {
        b = b.prop("iommu-map-mask", &cells(&[mask]));
    }
    b.end().end().token(END).finish()
}

fn map_of(blob: &[u8]) -> Result<Option<crate::arch::fdt::find::smmu::IommuMap>, FdtError> {
    pci_iommu_map(&Fdt::from_bytes(blob).unwrap())
}

#[test]
fn entries_offset_mask_and_first_match_win() {
    let blob = tree(&[0x100, 1, 0x40, 0x100, 0x100, 1, 0x9000, 0x10], Some(0xfff8), 1);
    let map = map_of(&blob).unwrap().unwrap();
    assert_eq!(map.stream_id(0x105, 1), Some(0x40), "masked to rid 0x100");
    assert_eq!(map.stream_id(0x1ff, 1), Some(0x138), "0x1f8 after the mask");
    assert_eq!(map.stream_id(0x0ff, 1), None);
    assert_eq!(map.stream_id(0x200, 1), None);
}

#[test]
fn a_map_that_cannot_be_trusted_is_refused() {
    let bad = |blob: Vec<u8>| assert_eq!(map_of(&blob), Err(FdtError::BadCells));
    bad(tree(&[0, 1, 0, 0x10], None, 2));
    bad(tree(&[0, 1, 0], None, 1));
    bad(tree(&[0xff00, 1, 0, 0x200], None, 1));
    bad(tree(&[0, 1, 0xffff_ff00, 0x200], None, 1));
    bad(tree(&[0; 36], None, 1));
    assert_eq!(map_of(&tree(&[0, 2, 0, 0x10], None, 1)), Err(FdtError::NoSuchNode));
}
