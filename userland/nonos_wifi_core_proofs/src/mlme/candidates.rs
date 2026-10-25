// NONOS Operating System (AGPL-3.0-or-later)

//! The candidates a join picks from: better security first, then signal, then
//! how often heard. A frame forged under a real access point's BSSID is a
//! candidate of its own and never replaces what the AP sent; a frame that
//! cannot be joined from is none, only the reason kept.

use nonos_wifi_core::bss::Candidates;
use nonos_wifi_core::mlme::MlmeFailure;
use nonos_wifi_core::rsn::{JoinPolicy, SelectError};

use super::candidates_bound::order;
use super::sim::{beacon, rsne};
use super::sim::{AP, OTHER};

pub(super) fn sae() -> Vec<u8> {
    rsne(&[8], 0x80, 4)
}

pub(super) fn psk() -> Vec<u8> {
    rsne(&[2], 0, 4)
}

#[test]
fn security_first_then_signal_then_how_often_heard() {
    let mut c = Candidates::new(b"home", JoinPolicy::ANY);
    let (wpa2, wpa3) = (beacon(OTHER, &[&psk()]), beacon(AP, &[&sae()]));
    c.hear(&wpa2, 1, 90);
    c.hear(&wpa3, 6, 20);
    assert_eq!(order(&mut c), vec![(wpa3.clone(), 6), (wpa2.clone(), 1)], "WPA3 though fainter");
    let mut c = Candidates::new(b"home", JoinPolicy::ANY);
    let (near, far) = (beacon(AP, &[&psk()]), beacon(OTHER, &[&psk()]));
    c.hear(&far, 11, 40);
    c.hear(&near, 1, 70);
    c.hear(&far, 11, 40);
    assert_eq!(order(&mut c)[0], (near, 1), "the stronger signal first");
    let mut c = Candidates::new(b"home", JoinPolicy::ANY);
    c.hear(&wpa2, 1, 50);
    c.hear(&far, 11, 50);
    c.hear(&far, 11, 50);
    assert_eq!(order(&mut c)[0].0, far, "heard more often");
}

#[test]
fn a_forged_frame_under_the_aps_bssid_never_replaces_the_aps() {
    let mut c = Candidates::new(b"home", JoinPolicy::ANY);
    let (real, forged) = (beacon(AP, &[&sae()]), beacon(AP, &[&psk()]));
    c.hear(&real, 6, 30);
    c.hear(&forged, 6, 99);
    assert_eq!(
        order(&mut c),
        vec![(real.clone(), 6), (forged.clone(), 6)],
        "two views, the AP's first"
    );
    // Even a forged view of the same security, louder, leaves the AP's to try next.
    let mut c = Candidates::new(b"home", JoinPolicy::ANY);
    let loud = beacon(AP, &[&rsne(&[8], 0x80 | 0x40, 4)]);
    c.hear(&real, 6, 30);
    c.hear(&loud, 6, 99);
    assert_eq!(order(&mut c)[1], (real, 6));
}

#[test]
fn a_frame_that_cannot_be_joined_from_is_no_candidate() {
    let mut c = Candidates::new(b"home", JoinPolicy::WPA3_ONLY);
    c.hear(&beacon(AP, &[&psk()]), 6, 50);
    assert!(!c.found());
    assert_eq!(c.refusal(), Some(MlmeFailure::Select(SelectError::Downgrade)));
    c.hear(&beacon(AP, &[]), 6, 50);
    assert_eq!(c.refusal(), Some(MlmeFailure::OpenNetwork));
    let mut c = Candidates::new(b"elsewhere", JoinPolicy::ANY);
    c.hear(&beacon(AP, &[&psk()]), 6, 50);
    assert!(!c.found() && c.refusal().is_none(), "another network's");
}
