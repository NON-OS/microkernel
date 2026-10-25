// NONOS Operating System (AGPL-3.0-or-later)
//! The `gic-version=2` and `iommu=smmuv3` trees: what differs is found.

use crate::arch::fdt::find::{gic, pci, timer};
use crate::arch::fdt::irq::{gic_edge, gic_intid};
use crate::arch::fdt::Fdt;
use crate::fixtures::{VIRT_GICV2, VIRT_SMMUV3};

#[test]
fn gicv2_reports_the_cpu_interface_not_a_redistributor() {
    let f = Fdt::from_bytes(VIRT_GICV2).unwrap();
    let g = gic::find(&f).unwrap().unwrap();
    assert_eq!(g.version, gic::GicVersion::V2);
    assert_eq!((g.dist_base, g.redist_or_cpu_base), (0x0800_0000, 0x0801_0000));
    assert!(f.find_path(b"/intc/v2m").unwrap().is_some());
    assert!(f.find_path(b"/intc/its").unwrap().is_none());
}

#[test]
fn gicv2_ppi_cpu_mask_is_not_a_trigger() {
    let f = Fdt::from_bytes(VIRT_GICV2).unwrap();
    let t = timer::find(&f).unwrap().unwrap();
    assert_eq!((t.nonsecure_phys_intid, t.virtual_intid), (30, 27));
    let node = f.find_path(b"/timer").unwrap().unwrap();
    let spec = f.interrupt(&node, 1).unwrap().unwrap();
    assert_eq!((gic_intid(spec.cells), gic_edge(spec.cells)), (Ok(30), Ok(false)));
}

#[test]
fn smmuv3_node_and_the_pci_iommu_map() {
    let f = Fdt::from_bytes(VIRT_SMMUV3).unwrap();
    let mut found = f.devices(&[b"arm,smmu-v3"]);
    let (smmu, _) = found.next().unwrap().unwrap();
    assert!(found.next().is_none());
    assert_eq!(f.mmio(&smmu, 0), Ok(Some((0x0905_0000, 0x2_0000))));
    let intids: Vec<_> =
        (0..4).map(|i| gic_intid(f.interrupt(&smmu, i).unwrap().unwrap().cells).unwrap()).collect();
    assert_eq!(intids, [106, 107, 108, 109]);
    let host = f.find_path(b"/pcie@10000000").unwrap().unwrap();
    let map = f.property(&host, b"iommu-map").unwrap().unwrap();
    assert_eq!(map.data, crate::builder::cells(&[0, 0x8005, 0, 0x1_0000]));
    assert_eq!(f.by_phandle(0x8005).unwrap().name, b"smmuv3@9050000");
    assert_eq!(f.by_phandle(0x8006).unwrap().name, b"pl061@9030000");
    assert!(pci::find(&f).unwrap().is_some());
}
