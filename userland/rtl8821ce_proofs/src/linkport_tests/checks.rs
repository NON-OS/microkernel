// NONOS Operating System (AGPL-3.0-or-later)

//! Frames the station refuses: unprotected, or ciphertext called plain.

use nonos_wifi_core::dot11::ccmp::encrypt;
use nonos_wifi_core::netif::LinkPort;

use super::card::Card;
use super::receive::{from_ap_plain, W0_ENC_AES, W0_SWDEC};
use super::rig::{eth_frame, link_with, AP_MAC, PTK, STA_MAC};
use super::rxbuf::RxBuf;

pub(super) fn with_fcs(mut f: Vec<u8>) -> Vec<u8> {
    f.extend_from_slice(&[0xFC; 4]);
    f
}

#[test]
fn an_unprotected_frame_is_refused_on_a_protected_link() {
    // What this fixture used to stage: plaintext data, as if decryption had
    // also cleared the Protected bit. On a protected link that is an
    // injection, and the stack must not see it.
    let eth = eth_frame(STA_MAC, AP_MAC, [0x08, 0x00], &[1, 2, 3, 4]);
    let mut rx = RxBuf::new(0x4000_0000);
    rx.stage(0, &with_fcs(from_ap_plain(&eth)));
    let (mut link, _tx) = link_with(Card::new(0, 1), rx);
    link.associate(AP_MAC, PTK);
    let mut out = [0u8; 1600];
    assert!(link.poll_rx(&mut out).is_none(), "refused");
    assert_eq!((link.stats().rx_refused, link.stats().rx_eth), (1, 0), "and counted");
}

#[test]
fn a_frame_the_chip_left_encrypted_is_decrypted_in_software() {
    // SWDEC set, or no security type at all: the frame is still ciphertext
    // and the station decrypts it under the pairwise key.
    for w0 in [W0_ENC_AES | W0_SWDEC, 0] {
        let eth = eth_frame(STA_MAC, AP_MAC, [0x08, 0x00], b"software");
        let enc = encrypt(&from_ap_plain(&eth), 5, &PTK).expect("protected");
        let mut rx = RxBuf::new(0x4000_0000);
        rx.stage_w0(0, &with_fcs(enc), w0);
        let (mut link, _tx) = link_with(Card::new(0, 1), rx);
        link.associate(AP_MAC, PTK);
        let mut out = [0u8; 1600];
        let n = link.poll_rx(&mut out).expect("decrypted and delivered");
        assert_eq!(&out[..n], &eth[..]);
    }
}

#[test]
fn ciphertext_the_chip_claims_to_have_decrypted_is_not_delivered() {
    // A frame the station would read as plaintext must really be plaintext:
    // ciphertext marked decrypted fails the LLC/SNAP check, never reaches the
    // stack as garbage.
    let eth = eth_frame(STA_MAC, AP_MAC, [0x08, 0x00], b"cipher");
    let enc = encrypt(&from_ap_plain(&eth), 5, &PTK).expect("protected");
    let mut rx = RxBuf::new(0x4000_0000);
    rx.stage_w0(0, &with_fcs(enc), W0_ENC_AES);
    let (mut link, _tx) = link_with(Card::new(0, 1), rx);
    link.associate(AP_MAC, PTK);
    let mut out = [0u8; 1600];
    assert!(link.poll_rx(&mut out).is_none());
}
