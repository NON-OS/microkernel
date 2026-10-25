// NONOS Operating System (AGPL-3.0-or-later)
//! The tree's reserved DMA ranges: keyed by who the unit sees (requester
//! id, a bridge's buses, streams), the child's `reg` as the physical base,
//! and what cannot be a range or be keyed said rather than guessed.

use crate::arch::fdt::find::reserved_dma::{Owner, TreeRange};
use crate::memory::iommu::reserved::ReservedRange;
use crate::reserved_dma_tree::{read, tree, BARE, BRIDGE, FUNCTION, PLATFORM};

const fn range(iova: u64, phys: u64, len: u64) -> ReservedRange {
    ReservedRange { iova, phys, len }
}

#[test]
fn each_entry_is_keyed_by_what_the_unit_sees() {
    let children = [
        (Some((0x9000_0000, 0x40_0000)), vec![PLATFORM, 0, 0x9000_0000, 0, 0x40_0000]),
        (Some((0xa000_0000, 0x1_0000)), vec![FUNCTION, 0x0200_0000, 0, 0x1000_0000, 0, 0x1_0000]),
        (Some((0xb000_0000, 0x2000)), vec![BRIDGE, 0, 0xb000_0000, 0, 0x2000]),
        (None, vec![PLATFORM, 0, 0x5000_0000, 0, 0x1000]),
    ];
    let (got, unkeyed) = read(&tree(&children, None)).unwrap();
    let streams = Owner::Streams { sids: [0x10, 0x11, 0, 0], count: 2 };
    let hole: Option<&[u8]> = Some(b"an IOVA hole with no physical range");
    assert_eq!(
        got,
        vec![
            TreeRange {
                owner: streams,
                range: range(0x9000_0000, 0x9000_0000, 0x40_0000),
                problem: None
            },
            TreeRange {
                owner: Owner::Pci { rid: 0x0113 },
                range: range(0x1000_0000, 0xa000_0000, 0x1_0000),
                problem: None
            },
            TreeRange {
                owner: Owner::PciBridge { first_bus: 0, last_bus: 0x3f },
                range: range(0xb000_0000, 0xb000_0000, 0x2000),
                problem: None
            },
            TreeRange { owner: streams, range: range(0x5000_0000, 0, 0x1000), problem: hole },
        ]
    );
    assert_eq!(unkeyed, None);
    assert!(got[0].range.is_identity() && !got[1].range.is_identity());
}

#[test]
fn what_cannot_be_keyed_is_left_out_and_said() {
    let bare = [(Some((0x9000_0000, 0x1000)), vec![BARE, 0, 0x9000_0000, 0, 0x1000])];
    let (got, unkeyed) = read(&tree(&bare, None)).unwrap();
    assert!(got.is_empty());
    assert_eq!(unkeyed, Some(&b"a device with no iommus to key it by"[..]));
    let bridge = [(Some((0xb000_0000, 0x2000)), vec![BRIDGE, 0, 0xb000_0000, 0, 0x2000])];
    let (got, unkeyed) = read(&tree(&bridge, Some(1))).unwrap();
    assert!(got.is_empty());
    assert_eq!(unkeyed, Some(&b"a PCI domain other than 0"[..]));
}
