// NONOS Operating System (AGPL-3.0-or-later)

//! A deauthentication from the access point, and from another.

use nonos_wifi_core::netif::LinkPort;

use super::card::Card;
use super::checks::with_fcs;
use super::rig::{link_with, AP_MAC, PTK, STA_MAC};
use super::rxbuf::RxBuf;

// A deauthentication from the AP to us with `reason`, and its FCS.
pub(super) fn deauth_from_ap(reason: u16) -> Vec<u8> {
    let mut f = vec![0xC0, 0x00, 0x00, 0x00];
    f.extend_from_slice(&STA_MAC);
    f.extend_from_slice(&AP_MAC);
    f.extend_from_slice(&AP_MAC);
    f.extend_from_slice(&[0x30, 0x00]);
    f.extend_from_slice(&reason.to_le_bytes());
    with_fcs(f)
}

#[test]
fn a_deauthentication_from_the_access_point_takes_the_link_down() {
    let mut rx = RxBuf::new(0x4000_0000);
    rx.stage(0, &deauth_from_ap(7));
    let (mut link, _tx) = link_with(Card::new(0, 1), rx);
    link.associate(AP_MAC, PTK);
    let mut out = [0u8; 1600];
    assert!(link.poll_rx(&mut out).is_none());
    assert!(!link.link_up(), "the link reads down");
    assert_eq!(link.take_left(), Some(7), "with the AP's reason, for the serve loop");
    assert_eq!(link.take_left(), None, "reported once");
}

#[test]
fn a_deauthentication_from_another_access_point_is_ignored() {
    let mut f = deauth_from_ap(7);
    f[10] ^= 0x01; // transmitter: someone else
    let mut rx = RxBuf::new(0x4000_0000);
    rx.stage(0, &f);
    let (mut link, _tx) = link_with(Card::new(0, 1), rx);
    link.associate(AP_MAC, PTK);
    let mut out = [0u8; 1600];
    assert!(link.poll_rx(&mut out).is_none());
    assert!(link.link_up() && link.take_left().is_none(), "the association stands");
}
