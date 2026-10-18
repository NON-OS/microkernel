// NONOS Operating System (AGPL-3.0-or-later)
//! Whole tables read into the records the broker takes from a tree.

use crate::arch::aarch64::iort::{root_complexes, RootComplex, MAX_ROOT_COMPLEXES};
use crate::arch::fdt::find::msi::MsiKind;
use crate::table::Table;

pub const ITS_BASE: u64 = 0x0808_0000;
pub fn its(id: u32) -> Option<u64> {
    (id == 0).then_some(ITS_BASE)
}

/// QEMU virt's shape: the root complex sends every requester id to the
/// SMMU as the same stream id, and the SMMU every stream id to the ITS as
/// the same device id.
#[test]
fn a_root_complex_behind_an_smmu() {
    let mut t = Table::new();
    let group = t.its_group(&[0]);
    let smmu = t.smmu(0x0905_0000, 1, [106, 0, 108, 0], 0, &[(0, 0x1_0000, 0, group, 0)]);
    t.root(0, &[(0, 0x1_0000, 0, smmu, 0)]);
    let [Some(rc), None, None, None] = root_complexes(&t.finish(), its).unwrap() else {
        panic!("one root complex")
    };
    let (unit, sids) = rc.smmu.unwrap();
    assert_eq!((unit.base, unit.size, unit.phandle), (0x0905_0000, 0x2_0000, smmu));
    assert_eq!((unit.coherent, unit.eventq_intid, unit.gerror_intid), (true, 106, 108));
    let (msi, dids) = rc.msi.unwrap();
    assert_eq!(
        (msi.kind, msi.base, msi.size, msi.phandle),
        (MsiKind::Its, ITS_BASE, 0x2_0000, group)
    );
    for rid in [0u16, 1, 0x0100, 0xFFFF] {
        assert_eq!(sids.stream_id(rid, smmu), Some(u32::from(rid)));
        assert_eq!(dids.output(rid, group), Some(u32::from(rid)));
        assert_eq!(sids.stream_id(rid, group), None);
    }
}

/// No SMMU: the root complex sends straight to the ITS group, and its
/// devices reach memory untranslated, as a tree without `iommu-map`.
#[test]
fn a_root_complex_straight_to_its() {
    let mut t = Table::new();
    let group = t.its_group(&[0, 1]);
    t.root(0, &[(0, 0x100, 0x4000, group, 0)]);
    t.root(1, &[(0, 0x100, 0x5000, group, 0)]);
    let found: [Option<RootComplex>; MAX_ROOT_COMPLEXES] =
        root_complexes(&t.finish(), its).unwrap();
    assert!(found[2..].iter().all(Option::is_none));
    for (n, base) in [(0, 0x4000), (1, 0x5000)] {
        let rc = found[n].unwrap();
        assert_eq!((rc.segment, rc.smmu), (n as u32, None));
        let (msi, dids) = rc.msi.unwrap();
        // Never a v2m frame: IORT routes to ITS groups alone.
        assert_ne!(msi.kind, MsiKind::V2m { spi_base: 0, spi_count: 0 });
        assert_eq!(msi.base, ITS_BASE);
        assert_eq!(dids.output(0x42, group), Some(base + 0x42));
        assert_eq!(dids.output(0x100, group), None);
    }
}
