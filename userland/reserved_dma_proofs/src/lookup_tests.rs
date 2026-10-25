// NONOS Operating System (AGPL-3.0-or-later)
//! A device's ranges out of the tree's entries: its own and its host
//! bridge's, by requester id; a platform device's by any of its streams;
//! one that cannot be a range refuses the lot; segment 0 only.

use crate::arch::fdt::find::reserved_dma::{Owner, TreeRange};
use crate::lookup::collect;
use crate::memory::iommu::reserved::{ReservedRange, MAX_RESERVED};
use crate::memory::iommu::DeviceAddress;

fn entry(owner: Owner, iova: u64, problem: Option<&'static [u8]>) -> TreeRange {
    TreeRange { owner, range: ReservedRange { iova, phys: iova, len: 0x1000 }, problem }
}

#[test]
fn each_device_gets_what_names_it() {
    let streams = Owner::Streams { sids: [0x10, 0x11, 0, 0], count: 2 };
    let entries = [
        entry(Owner::Pci { rid: 0x0113 }, 0x1000, None),
        entry(Owner::PciBridge { first_bus: 0, last_bus: 1 }, 0x2000, None),
        entry(streams, 0x3000, None),
        entry(Owner::Pci { rid: 0x0200 }, 0x4000, None),
    ];
    let iovas = |d| collect(&entries, d).ranges().iter().map(|r| r.iova).collect::<Vec<_>>();
    assert_eq!(iovas(DeviceAddress::pci(1, 2, 3)), vec![0x1000, 0x2000]);
    assert_eq!(iovas(DeviceAddress::pci(1, 0, 0)), vec![0x2000]);
    assert_eq!(iovas(DeviceAddress::pci(2, 0, 0)), vec![0x4000]);
    assert_eq!(iovas(DeviceAddress::pci_in(1, 1, 2, 3)), Vec::<u64>::new());
    assert_eq!(iovas(DeviceAddress::stream(0x11)), vec![0x3000]);
    assert_eq!(iovas(DeviceAddress::stream(0x00)), Vec::<u64>::new());
    assert_eq!(iovas(DeviceAddress::stream(0x12)), Vec::<u64>::new());
}

#[test]
fn a_bad_entry_or_too_many_refuses_the_lot() {
    let bad =
        [entry(Owner::Pci { rid: 1 }, 0x1000, None), entry(Owner::Pci { rid: 1 }, 0, Some(b"x"))];
    assert_eq!(
        collect(&bad, DeviceAddress::Pci { segment: 0, rid: 1 }).unresolved,
        Some(&b"x"[..])
    );
    let many: Vec<_> =
        (0..=MAX_RESERVED as u64).map(|i| entry(Owner::Pci { rid: 1 }, i << 12, None)).collect();
    let got = collect(&many, DeviceAddress::Pci { segment: 0, rid: 1 });
    assert!(got.unresolved.is_some() && got.ranges().len() == MAX_RESERVED);
    assert!(collect(&many[..MAX_RESERVED], DeviceAddress::Pci { segment: 0, rid: 1 })
        .unresolved
        .is_none());
}
