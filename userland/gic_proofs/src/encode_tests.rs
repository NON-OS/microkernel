// NONOS Operating System (AGPL-3.0-or-later)
//! GICv2 acknowledge decode and SGI encodings.

use crate::gic::v2::encode::{iar_intid, sgir_all_others, sgir_to};

#[test]
fn special_intids_are_never_acknowledged() {
    assert_eq!(iar_intid(0), Some(0));
    assert_eq!(iar_intid(1019), Some(1019));
    for special in 1020..=1023 {
        assert_eq!(iar_intid(special), None);
    }
}

#[test]
fn an_sgis_sender_does_not_change_its_intid() {
    for cpu in 0..8u32 {
        for intid in 0..16u32 {
            assert_eq!(iar_intid((cpu << 10) | intid), Some(intid));
        }
    }
    assert_eq!(iar_intid((7 << 10) | 1023), None);
}

#[test]
fn every_sgi_to_every_target_list_round_trips() {
    for intid in 0..16u32 {
        for targets in 1..=255u8 {
            let v = sgir_to(targets, intid).unwrap();
            assert_eq!(v & 0xF, intid);
            assert_eq!((v >> 16) & 0xFF, targets as u32);
            assert_eq!(v >> 24, 0, "filter must be the target list");
            assert_eq!(v & 0xFFF0, 0, "reserved and NSATT bits stay clear");
        }
    }
}

#[test]
fn non_sgis_and_empty_lists_are_refused() {
    assert_eq!(sgir_to(0, 1), None);
    assert_eq!(sgir_to(1, 16), None);
    assert_eq!(sgir_all_others(16), None);
    assert_eq!(sgir_all_others(3), Some(0x0100_0003));
}
