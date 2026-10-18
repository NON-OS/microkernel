// NONOS Operating System (AGPL-3.0-or-later)
//! A domain's reserved ranges against a model of the domain, over generated
//! attach, failed-attach and detach orders: a range is mapped exactly while
//! some attached device names it, mapped once, and unmapped at the detach
//! of the last device naming it, so no other device can reach it after.

use std::collections::BTreeSet;

use crate::held::{name, unheld, unname, Held};
use crate::memory::iommu::reserved::ReservedRange;
use crate::memory::iommu::DeviceAddress;

fn range(i: u64) -> ReservedRange {
    ReservedRange { iova: i << 20, phys: i << 20, len: 0x1000 }
}

#[test]
fn a_range_is_mapped_exactly_while_an_attached_device_names_it() {
    let mut seed = 41u64;
    let mut next = |bound: u64| {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (seed >> 33) % bound
    };
    for _ in 0..20_000 {
        let devices: Vec<(DeviceAddress, Vec<ReservedRange>)> = (0..5)
            .map(|d| (DeviceAddress::stream(d), (0..next(4)).map(|_| range(next(6))).collect()))
            .collect();
        let (mut held, mut mapped, mut attached) =
            (Vec::<Held>::new(), BTreeSet::new(), BTreeSet::new());
        for _ in 0..40 {
            let d = next(5) as usize;
            let (address, ranges) = &devices[d];
            if !attached.contains(&d) {
                for r in unheld(&held, ranges) {
                    assert!(mapped.insert(r.iova), "mapped twice");
                }
                name(&mut held, ranges, *address);
                attached.insert(d);
                // A failed attach takes back exactly what this one added.
                if next(4) == 0 {
                    attached.remove(&d);
                    unname(&mut held, *address)
                        .iter()
                        .for_each(|r| assert!(mapped.remove(&r.iova)));
                }
            } else {
                attached.remove(&d);
                unname(&mut held, *address).iter().for_each(|r| assert!(mapped.remove(&r.iova)));
            }
            let want: BTreeSet<u64> =
                attached.iter().flat_map(|&a| devices[a].1.iter().map(|r| r.iova)).collect();
            assert_eq!(mapped, want, "mapped is not what the attached devices name");
            for h in &held {
                let users: BTreeSet<_> = h.users.iter().collect();
                let naming: BTreeSet<_> = attached
                    .iter()
                    .filter(|&&a| devices[a].1.contains(&h.range))
                    .map(|&a| &devices[a].0)
                    .collect();
                assert_eq!(users, naming, "users are not the attached devices naming it");
            }
        }
    }
}
