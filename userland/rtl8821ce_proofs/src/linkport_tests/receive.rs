// NONOS Operating System (AGPL-3.0-or-later)

//! Frames received, decrypted by the chip or not, back as Ethernet.

use nonos_wifi_core::netif::LinkPort;

use super::card::Card;
use super::rig::{eth_frame, link_with, AP_MAC, PTK, STA_MAC};
use super::rxbuf::RxBuf;

/// RX descriptor word 0: security type AES (`RX_DESC_ENC_AES`, bits 20-22).
pub(super) const W0_ENC_AES: u32 = 4 << 20;

/// RX descriptor word 0: the chip left decryption to software.
pub(super) const W0_SWDEC: u32 = 1 << 27;

// The access point's frame for `eth` (sent to us), as this chip hands it up
// after decrypting it in place: FromDS, Protected, the CCMP header (packet
// number `pn`), the plaintext, the MIC and the FCS still on.
pub(super) fn chip_decrypted(eth: &[u8], pn: u8) -> Vec<u8> {
    let mut f = vec![0x08, 0x42, 0, 0];
    f.extend_from_slice(&eth[0..6]); // addr1: the destination, us
    f.extend_from_slice(&AP_MAC); // addr2: the BSSID
    f.extend_from_slice(&eth[6..12]); // addr3: the source
    f.extend_from_slice(&[0x20, 0x00]);
    f.extend_from_slice(&[pn, 0, 0, 0x20, 0, 0, 0, 0]);
    f.extend_from_slice(&[0xAA, 0xAA, 0x03, 0x00, 0x00, 0x00]);
    f.extend_from_slice(&eth[12..]);
    f.extend_from_slice(&[0x4D; 8]); // MIC
    f.extend_from_slice(&[0xFC; 4]); // FCS
    f
}

#[test]
fn receive_recovers_the_original_ethernet_frame() {
    // The AP sends an Ethernet packet to us; the chip decrypts it in place.
    let eth = eth_frame(STA_MAC, AP_MAC, [0x08, 0x06], &[0xDE, 0xAD, 0xBE, 0xEF]);
    let mpdu = chip_decrypted(&eth, 1);

    // Stage it in RX slot 0 and tell the card one frame is ready.
    let mut rx = RxBuf::new(0x4000_0000);
    rx.stage_w0(0, &mpdu, W0_ENC_AES);
    let (mut link, _tx) = link_with(Card::new(0, 1), rx);
    link.associate(AP_MAC, PTK);

    let mut out = [0u8; 1600];
    let n = link.poll_rx(&mut out).expect("a staged frame is delivered");
    assert_eq!(&out[..n], &eth[..], "the exact Ethernet frame is recovered");
}

#[test]
fn receive_on_an_empty_ring_yields_nothing() {
    let (mut link, _tx) = link_with(Card::new(0, 0), RxBuf::new(0x4000_0000));
    link.associate(AP_MAC, PTK);
    let mut out = [0u8; 1600];
    assert!(link.poll_rx(&mut out).is_none(), "no frame queued means no delivery");
}

// The access point's frame for `eth` in the clear: FromDS, not protected.
pub(super) fn from_ap_plain(eth: &[u8]) -> Vec<u8> {
    let mut f = vec![0x08, 0x02, 0, 0];
    f.extend_from_slice(&eth[0..6]);
    f.extend_from_slice(&AP_MAC);
    f.extend_from_slice(&eth[6..12]);
    f.extend_from_slice(&[0x10, 0x00]);
    f.extend_from_slice(&[0xAA, 0xAA, 0x03, 0x00, 0x00, 0x00]);
    f.extend_from_slice(&eth[12..]);
    f
}
