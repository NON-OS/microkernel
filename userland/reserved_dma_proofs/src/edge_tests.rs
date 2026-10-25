// NONOS Operating System (AGPL-3.0-or-later)
//! `admit` at the edges the ruling names: a range filling its region, one a
//! page past it, a wrap, a held span just touched and just met, two new
//! ranges over each other, and nothing to admit.

use crate::memory::iommu::reserved::{admit, Refusal, ReservedRange};

#[test]
fn the_edges() {
    let r = |iova, phys, len| ReservedRange { iova, phys, len };
    let mem = [(0x8000_0000, 0x10_0000)];
    assert_eq!(admit(&[r(0x8000_0000, 0x8000_0000, 0x10_0000)], &[], &mem), Ok(()));
    assert_eq!(
        admit(&[r(0x8000_0000, 0x8000_1000, 0x10_0000)], &[], &mem),
        Err(Refusal::OutsideReservedMemory)
    );
    assert_eq!(
        admit(&[r(u64::MAX - 0xfff, 0x8000_0000, 0x2000)], &[], &mem),
        Err(Refusal::Malformed)
    );
    assert_eq!(admit(&[r(0x1000, 0x8000_0000, 0x1000)], &[(0x2000, 0x3000)], &mem), Ok(()));
    assert_eq!(
        admit(&[r(0x1000, 0x8000_0000, 0x2000)], &[(0x2000, 0x3000)], &mem),
        Err(Refusal::Overlap)
    );
    let two = [r(0x1000, 0x8000_0000, 0x2000), r(0x2000, 0x8000_2000, 0x1000)];
    assert_eq!(admit(&two, &[], &mem), Err(Refusal::Overlap));
    assert_eq!(admit(&[], &[(0, u64::MAX)], &[]), Ok(()));
}
