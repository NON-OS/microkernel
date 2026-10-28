// NONOS Operating System (AGPL-3.0-or-later)

//! The access point's record and the station's address on the model: a
//! record made when the station points at an AP and removed when it points
//! away; an address changed by taking the interface down and up again,
//! never under an association; and an association told and ended.

use nonos_wifi_core::regdom::Band;
use nonos_wifi_core::softmac::Radio;

use crate::model::net::{assoc, AP};
use crate::model::rig::up;

fn uni(c: &crate::model::Shared, cid: u16) -> Vec<Vec<u8>> {
    c.borrow().mcu.uni(cid).iter().map(|s| s.body.clone()).collect()
}

#[test]
fn pointing_at_an_ap_makes_its_record_and_pointing_away_removes_it() {
    let (mut r, c) = up();
    r.tune(Band::Ghz2, 6);
    r.set_bss(Some(AP));
    r.set_bss(None);
    let recs = uni(&c, 0x03);
    assert_eq!(recs.len(), 2);
    assert_eq!((recs[0][1], recs[0][8 + 8], &recs[0][8 + 12..8 + 18]), (1, 2, &AP[..]));
    assert_eq!((recs[1][2], recs[1][8 + 8]), (2, 0), "removed: disconnected");
    assert_eq!(c.borrow().mcu.roc, None, "its channel given back");
}

#[test]
fn an_address_changes_as_an_interface_goes_down_and_up() {
    let (mut r, c) = up();
    let new = [0x06, 1, 2, 3, 4, 5];
    let before = uni(&c, 0x01).len();
    assert!(r.set_mac(new) && r.mac() == new);
    let dev = uni(&c, 0x01);
    assert_eq!(dev.len(), before + 2);
    assert_eq!(
        (dev[before][8], dev[before + 1][8]),
        (0, 1),
        "down under the old, up under the new"
    );
    assert_eq!(&dev[before + 1][10..16], &new);
    r.tune(Band::Ghz2, 6);
    r.associated(Some(&assoc()));
    assert!(!r.set_mac([0x0e; 6]), "never under an association");
}

#[test]
fn an_association_connects_the_bss_on_its_channel_and_its_end_disconnects() {
    let (mut r, c) = up();
    r.tune(Band::Ghz2, 6);
    r.set_bss(Some(AP));
    r.associated(Some(&assoc()));
    let ch = c.borrow();
    assert_eq!((ch.mcu.bss, ch.mcu.roc), (Some(6), None), "the BSS's context holds channel 6");
    let rec = ch.mcu.uni(0x03).last().unwrap().body.clone();
    assert_eq!((rec[1], rec[5], rec[8]), (19, 0xe, 13), "the own entry pointed at the BSS");
    drop(ch);
    r.associated(None);
    assert_eq!(c.borrow().mcu.bss, None);
    assert!(r.tune(Band::Ghz2, 1), "and the radio tunes freely again");
}
