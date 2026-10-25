// NONOS Operating System (AGPL-3.0-or-later)
//! Requester ids carried through an SMMU whose mappings split them.

use crate::arch::aarch64::iort::root_complexes;
use crate::records_tests::its;
use crate::table::Table;

/// Requester ids 0-0xFF reach the SMMU as stream ids 0x1000-0x10FF. The
/// SMMU sends the first half to device ids from 0x9000, the second from
/// 0x50, and its own MSIs (mapping 0, the DeviceID index) elsewhere. Every
/// requester id is checked against the arithmetic, one by one.
#[test]
fn split_ranges_compose_exactly() {
    let mut t = Table::new();
    let group = t.its_group(&[0]);
    let own = (0, 1, 0x7777, group, 1);
    let maps = [own, (0x1080, 0x80, 0x50, group, 0), (0x1000, 0x80, 0x9000, group, 0)];
    let smmu = t.smmu(0x2B40_0000, 1 << 4, [0; 4], 0, &maps);
    t.root(0, &[(0, 0x100, 0x1000, smmu, 0)]);
    let rc = root_complexes(&t.finish(), its).unwrap()[0].unwrap();
    let (sids, dids) = (rc.smmu.unwrap().1, rc.msi.unwrap().1);
    assert_eq!(dids.count, 2);
    for rid in 0u16..=0x1FF {
        let r = u32::from(rid);
        let (want_sid, want_did) = match r {
            0..=0x7F => (Some(0x1000 + r), Some(0x9000 + r)),
            0x80..=0xFF => (Some(0x1000 + r), Some(0x50 + r - 0x80)),
            _ => (None, None),
        };
        assert_eq!(sids.stream_id(rid, smmu), want_sid, "rid {rid:#x}");
        assert_eq!(dids.output(rid, group), want_did, "rid {rid:#x}");
    }
}

/// An SMMU mapping that overlaps none of the root complex's stream ids
/// adds nothing; one that overlaps part adds that part.
#[test]
fn only_the_overlap_is_carried() {
    let mut t = Table::new();
    let group = t.its_group(&[0]);
    let maps = [(0x0, 0x10, 0x100, group, 0), (0x18, 0x10, 0x200, group, 0)];
    let smmu = t.smmu(0x2B40_0000, 0, [0; 4], 0, &maps);
    t.root(0, &[(0, 0x10, 0x10, smmu, 0)]);
    let rc = root_complexes(&t.finish(), its).unwrap()[0].unwrap();
    let dids = rc.msi.unwrap().1;
    assert_eq!(dids.count, 1);
    assert_eq!(dids.output(0x7, group), None);
    assert_eq!(dids.output(0x8, group), Some(0x200));
    assert_eq!(dids.output(0xF, group), Some(0x207));
}
