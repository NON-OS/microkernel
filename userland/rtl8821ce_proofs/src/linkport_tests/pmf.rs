// NONOS Operating System (AGPL-3.0-or-later)

//! Leaving, and deauthentications under management frame protection.

use nonos_wifi_core::dot11::ccmp::decrypt;
use nonos_wifi_core::netif::LinkPort;

use super::card::Card;
use super::deauth::deauth_from_ap;
use super::group::supplicant;
use super::rig::{link_with, AP_MAC, PTK, STA_MAC};
use super::rxbuf::RxBuf;
use crate::tx::desc::TXDESC_LEN;
use crate::tx::regs::REG_TXBD_IDX_BEQ;

#[test]
fn with_management_frame_protection_an_unprotected_deauthentication_is_ignored() {
    let mut rx = RxBuf::new(0x4000_0000);
    rx.stage(0, &deauth_from_ap(7));
    let (mut link, _tx) = link_with(Card::new(0, 1), rx);
    link.associate(AP_MAC, PTK);
    link.set_supplicant(supplicant(true));
    let mut out = [0u8; 1600];
    assert!(link.poll_rx(&mut out).is_none());
    assert!(link.link_up() && link.take_left().is_none(), "a forgery cannot end the link");
}

#[test]
fn leaving_tells_the_access_point_reason_3() {
    let card = Card::new(0, 0);
    let probe = card.clone();
    let (mut link, tx) = link_with(card, RxBuf::new(0x4000_0000));
    link.send_deauth();
    assert!(!probe.wrote(REG_TXBD_IDX_BEQ), "nothing is sent before an association");
    link.associate(AP_MAC, PTK);
    link.send_deauth();
    let mem = tx.mem.borrow();
    let f = &mem[TXDESC_LEN..TXDESC_LEN + 26];
    assert_eq!(f[0], 0xC0, "a deauthentication");
    assert_eq!(&f[4..10], &AP_MAC, "to the access point");
    assert_eq!(&f[10..16], &STA_MAC, "from us");
    assert_eq!(&f[24..26], &[3, 0], "reason 3: the station is leaving");
    drop(mem);
    assert!(probe.wrote(REG_TXBD_IDX_BEQ), "and the queue is kicked");
}

#[test]
fn with_management_frame_protection_the_deauthentication_goes_protected() {
    let (mut link, tx) = link_with(Card::new(0, 0), RxBuf::new(0x4000_0000));
    link.associate(AP_MAC, PTK);
    link.set_supplicant(supplicant(true));
    link.send_deauth();
    let mem = tx.mem.borrow();
    let f = &mem[TXDESC_LEN..TXDESC_LEN + 24 + 8 + 2 + 8];
    assert_eq!(f[0], 0xC0);
    assert_ne!(f[1] & 0x40, 0, "the Protected bit is set");
    assert_eq!(decrypt(f, &PTK).expect("under the pairwise key"), vec![3, 0], "reason 3 inside");
}
