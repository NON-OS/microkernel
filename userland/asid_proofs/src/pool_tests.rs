// NONOS Operating System (AGPL-3.0-or-later)
//! The pool's rules, each over every ASID an 8-bit core has.

use crate::pool::{Pool, Tag};

fn own(tag: Tag) -> u16 {
    match tag {
        Tag::Own(asid) => asid,
        Tag::Shared(_) => panic!("shared where an ASID was free"),
    }
}

/// 254 roots on an 8-bit core each get their own ASID: never 0, the
/// kernel's, never 255, the shared one, and no two alike. The 255th gets
/// the shared one.
#[test]
fn every_root_its_own_until_none_is_left() {
    let mut pool = Pool::new(8);
    let mut seen = [false; 256];
    for root in 1..=254u64 {
        let asid = own(pool.tag(root << 12));
        assert!((1..=254).contains(&asid));
        assert!(!seen[usize::from(asid)], "ASID {asid} handed out twice");
        seen[usize::from(asid)] = true;
    }
    assert_eq!(pool.tag(255 << 12), Tag::Shared(255));
    assert_eq!(pool.tag(256 << 12), Tag::Shared(255));
}

/// A root keeps its ASID across switches: no switch reallocates it.
#[test]
fn a_root_keeps_its_asid() {
    let mut pool = Pool::new(8);
    let a = own(pool.tag(0x1000));
    let _ = pool.tag(0x2000);
    assert_eq!(pool.tag(0x1000), Tag::Own(a));
}

/// A retired root's ASID goes to the next new root; retiring twice, or a
/// root never tagged, gives nothing back.
#[test]
fn retired_asids_come_back() {
    let mut pool = Pool::new(8);
    for root in 1..=254u64 {
        pool.tag(root << 12);
    }
    let freed = own(pool.tag(7 << 12));
    assert_eq!(pool.retire(7 << 12), Some(freed));
    assert_eq!(pool.retire(7 << 12), None);
    assert_eq!(pool.retire(999 << 12), None);
    assert_eq!(pool.tag(1000 << 12), Tag::Own(freed));
    assert_eq!(pool.tag(1001 << 12), Tag::Shared(255));
}

/// A 16-bit core: the shared ASID is 0xFFFF and the first is 1.
#[test]
fn sixteen_bits() {
    let mut pool = Pool::new(16);
    assert_eq!(pool.tag(0x1000), Tag::Own(1));
    for root in 2..=0xFFFEu64 {
        pool.tag(root << 12);
    }
    assert_eq!(pool.tag(0xFFFF << 12), Tag::Shared(0xFFFF));
}
