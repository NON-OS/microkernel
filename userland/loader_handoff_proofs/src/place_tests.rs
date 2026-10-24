// NONOS Operating System (AGPL-3.0-or-later)
//! Where a handoff may lie, against the rule written out: non-empty, eight
//! byte aligned, not at zero, and wholly inside one region the tree names.

use crate::place::{placed, HANDOFF_ALIGN};

fn model(regions: &[(u64, u64)], at: u64, len: u64) -> bool {
    let Some(end) = at.checked_add(len) else { return false };
    at != 0
        && len != 0
        && at.is_multiple_of(HANDOFF_ALIGN)
        && regions.iter().any(|&(b, s)| b.checked_add(s).is_some_and(|t| b <= at && end <= t))
}

#[test]
fn agrees_with_the_rule_over_generated_maps() {
    let mut x = 0x9e37_79b9_7f4a_7c15u64;
    let mut next = || {
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        x
    };
    for _ in 0..200_000 {
        let n = (next() % 4) as usize;
        let regions: Vec<(u64, u64)> =
            (0..n).map(|_| ((next() % 64) * 0x1000, (next() % 32) * 0x800)).collect();
        let at = (next() % 64) * 0x1000 + (next() % 3) * 4;
        let len = next() % 0x3000;
        assert_eq!(
            placed(regions.iter().copied(), at, len),
            model(&regions, at, len),
            "{regions:x?} {at:#x}+{len:#x}"
        );
    }
}

#[test]
fn the_edges() {
    let ram = [(0x4000_0000u64, 0x1000_0000u64)];
    let r = || ram.iter().copied();
    assert!(placed(r(), 0x4000_0000, 0x200), "at the region's start");
    assert!(placed(r(), 0x5000_0000 - 0x200, 0x200), "ending at its end");
    assert!(!placed(r(), 0x5000_0000 - 0x1f8, 0x200), "one word past its end");
    assert!(!placed(r(), 0x3fff_fff8, 0x200), "starting before it");
    assert!(!placed(r(), 0x4000_0004, 0x200), "misaligned");
    assert!(!placed(r(), 0x4000_0000, 0), "empty");
    assert!(!placed(r(), u64::MAX - 7, 0x200), "wrapping");
    assert!(!placed([(0, 0x1000)].into_iter(), 0, 0x200), "at zero, which is no loader");
    let wrapping_region = [(u64::MAX - 0xfff, 0x2000)];
    assert!(!placed(wrapping_region.iter().copied(), u64::MAX - 0xff7, 8), "a region that wraps");
    let split = [(0x4000_0000, 0x1000), (0x4000_1000, 0x1000)];
    assert!(!placed(split.iter().copied(), 0x4000_0f00, 0x200), "across two regions");
}
