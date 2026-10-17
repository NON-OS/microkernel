// NONOS Operating System (AGPL-3.0-or-later)

//! Data frames from the BSS taken, and the rest dropped with a reason.

use nonos_wifi_core::station::{LinkStation, Rx, RxDrop};

use super::frames::{from_ap, ip_body, SRC, STA};
use super::stations::station;
use super::vectors::tk;

#[test]
fn a_protected_frame_from_the_bss_becomes_ethernet() {
    let mut s = station();
    for qos in [false, true] {
        let f = from_ap(STA, &ip_body(b"payload"), Some(&tk()), if qos { 2 } else { 1 }, 0, qos);
        let Rx::Ethernet(eth) = s.receive(&f, false) else { panic!("delivered, qos={qos}") };
        assert_eq!(&eth[0..6], &STA, "destination");
        assert_eq!(&eth[6..12], &SRC, "source is the FromDS Address 3");
        assert_eq!(&eth[12..14], &[0x08, 0x00]);
        assert_eq!(&eth[14..], b"payload");
    }
}

#[test]
fn a_chip_decrypted_frame_loses_its_ccmp_header_and_mic() {
    let mut s = station();
    // The chip hands up the frame decrypted in place: CCMP header, plaintext, MIC.
    let mut f = from_ap(STA, &[], None, 0, 0, true);
    f[1] |= 0x40;
    f.extend_from_slice(&[5, 0, 0, 0x20, 0, 0, 0, 0]);
    f.extend_from_slice(&ip_body(b"chip"));
    f.extend_from_slice(&[0xEE; 8]);
    let Rx::Ethernet(eth) = s.receive(&f, true) else { panic!("delivered") };
    assert_eq!(&eth[14..], b"chip", "no MIC bytes trail the payload");
    assert!(matches!(s.receive(&f, true), Rx::Dropped(RxDrop::Replay)), "replay checked too");
}

#[test]
fn frames_that_are_not_for_this_link_are_dropped_with_a_reason() {
    let mut s = station();
    let good = from_ap(STA, &ip_body(b"x"), Some(&tk()), 1, 0, true);
    let mut other_bss = good.clone();
    other_bss[10..16].copy_from_slice(&[0x02, 0, 0, 0, 0, 0x55]);
    assert!(matches!(s.receive(&other_bss, false), Rx::Dropped(RxDrop::NotFromBss)));
    let mut to_ds = good.clone();
    to_ds[1] = (to_ds[1] & !0x03) | 0x01;
    assert!(matches!(s.receive(&to_ds, false), Rx::Dropped(RxDrop::NotFromBss)));
    let mut other_sta = good.clone();
    other_sta[4..10].copy_from_slice(&[0x02, 0, 0, 0, 0, 0x33]);
    assert!(matches!(s.receive(&other_sta, false), Rx::Dropped(RxDrop::NotForUs)));
    let mut fragment = good.clone();
    fragment[1] |= 0x04;
    assert!(matches!(s.receive(&fragment, false), Rx::Dropped(RxDrop::Fragmented)));
    let mut amsdu = good.clone();
    amsdu[24] |= 0x80;
    assert!(matches!(s.receive(&amsdu, false), Rx::Dropped(RxDrop::Amsdu)));
    let plain = from_ap(STA, &ip_body(b"injected"), None, 0, 0, true);
    assert!(matches!(s.receive(&plain, false), Rx::Dropped(RxDrop::Unprotected)));
    let mut tampered = good.clone();
    let last = tampered.len() - 1;
    tampered[last] ^= 1;
    assert!(matches!(s.receive(&tampered, false), Rx::Dropped(RxDrop::Undecryptable)));
    assert!(matches!(s.receive(&good[..30], false), Rx::Dropped(RxDrop::Malformed)));
    assert!(matches!(s.receive(&good, false), Rx::Ethernet(_)));
    assert!(matches!(s.receive(&good, false), Rx::Dropped(RxDrop::Replay)));
    let mut unassociated = LinkStation::new(STA);
    assert!(matches!(unassociated.receive(&good, false), Rx::Dropped(RxDrop::NotAssociated)));
}
