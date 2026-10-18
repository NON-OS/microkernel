// NONOS Operating System (AGPL-3.0-or-later)

//! Replays refused, oversize frames dropped, raw frames without FCS.

use nonos_wifi_core::netif::LinkPort;

use super::card::Card;
use super::receive::{chip_decrypted, W0_ENC_AES};
use super::rig::{eth_frame, link_with, AP_MAC, PTK, STA_MAC};
use super::rxbuf::RxBuf;

#[test]
fn a_replayed_frame_is_refused() {
    let eth = eth_frame(STA_MAC, AP_MAC, [0x08, 0x00], &[9, 9]);
    let f = chip_decrypted(&eth, 1);
    let mut rx = RxBuf::new(0x4000_0000);
    rx.stage_w0(0, &f, W0_ENC_AES);
    rx.stage_w0(1, &f, W0_ENC_AES);
    let (mut link, _tx) = link_with(Card::new(0, 2), rx);
    link.associate(AP_MAC, PTK);
    let mut out = [0u8; 1600];
    assert!(link.poll_rx(&mut out).is_some(), "the first copy is delivered");
    assert!(link.poll_rx(&mut out).is_none(), "the replay of packet number 1 is not");
    assert_eq!(link.stats().rx_refused, 1);
}

#[test]
fn a_frame_larger_than_the_caller_buffer_is_dropped_not_cut() {
    let eth = eth_frame(STA_MAC, AP_MAC, [0x08, 0x00], &[7; 64]);
    let mut rx = RxBuf::new(0x4000_0000);
    rx.stage_w0(0, &chip_decrypted(&eth, 1), W0_ENC_AES);
    let (mut link, _tx) = link_with(Card::new(0, 1), rx);
    link.associate(AP_MAC, PTK);
    let mut out = [0u8; 32];
    assert!(link.poll_rx(&mut out).is_none(), "no truncated packet reaches the stack");
    assert_eq!(link.stats().rx_refused, 1);
}

#[test]
fn poll_raw_hands_up_the_frame_without_its_fcs() {
    let beacon = [0x80u8; 40];
    let mut rx = RxBuf::new(0x4000_0000);
    rx.stage(0, &beacon);
    rx.stage(1, &[0x80, 0x00, 0x00]);
    let (mut link, _tx) = link_with(Card::new(0, 2), rx);
    let mut out = [0u8; 1600];
    assert_eq!(link.poll_raw(&mut out), Some(36), "four bytes of FCS dropped");
    assert_eq!(link.poll_raw(&mut out), None, "a frame shorter than an FCS is dropped");
}
