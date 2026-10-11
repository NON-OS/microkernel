// NONOS Operating System (AGPL-3.0-or-later)
//! The MSI controller a host bridge names, from the boards' own trees.

use crate::arch::fdt::find::msi::{pci_msi_map, MsiController, MsiKind};
use crate::arch::fdt::Fdt;
use crate::fixtures::{RPI4B, VIRT_GICV2, VIRT_GICV3, VIRT_SMMUV3};

#[test]
fn virt_gicv3_routes_every_rid_to_its_its_as_its_own_device_id() {
    for blob in [VIRT_GICV3, VIRT_SMMUV3] {
        let (its, map) = pci_msi_map(&Fdt::from_bytes(blob).unwrap()).unwrap().unwrap();
        let want = MsiController {
            kind: MsiKind::Its,
            base: 0x0808_0000,
            size: 0x2_0000,
            phandle: 0x8004,
        };
        assert_eq!(its, want);
        assert_eq!(map.output(0x0008, 0x8004), Some(0x0008));
        assert_eq!(map.output(0xffff, 0x8004), Some(0xffff));
        assert_eq!(map.output(0x0008, 0x8005), None, "not the SMMU's phandle");
    }
}

#[test]
fn virt_gicv2_names_a_v2m_frame_that_reports_its_own_spis() {
    let (v2m, map) = pci_msi_map(&Fdt::from_bytes(VIRT_GICV2).unwrap()).unwrap().unwrap();
    let kind = MsiKind::V2m { spi_base: 0, spi_count: 0 };
    assert_eq!(v2m, MsiController { kind, base: 0x0802_0000, size: 0x1000, phandle: 0x8004 });
    assert_eq!(map.output(0x0010, 0x8004), Some(0x0010));
}

#[test]
fn a_board_with_no_ecam_host_has_no_msi_map() {
    assert_eq!(pci_msi_map(&Fdt::from_bytes(RPI4B).unwrap()), Ok(None));
}
