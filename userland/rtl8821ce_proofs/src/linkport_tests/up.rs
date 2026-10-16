// NONOS Operating System (AGPL-3.0-or-later)

//! Down until associated; up, the MAC published and frames encrypted.

use nonos_wifi_core::netif::LinkPort;

use super::card::Card;
use super::rig::{eth_frame, link_with, AP_MAC, PTK, STA_MAC};
use super::rxbuf::RxBuf;
use crate::tx::desc::TXDESC_LEN;
use crate::tx::regs::REG_TXBD_IDX_BEQ;

#[test]
fn an_unassociated_link_is_down_with_no_mac() {
    let (link, _tx) = link_with(Card::new(0, 0), RxBuf::new(0x4000_0000));
    assert!(!link.link_up(), "no association means link down");
    assert_eq!(link.mac(), None, "no MAC is offered until associated");
}

#[test]
fn an_unassociated_link_refuses_to_transmit() {
    let (mut link, _tx) = link_with(Card::new(0, 0), RxBuf::new(0x4000_0000));
    let eth = [0u8; 64];
    assert!(!link.send_tx(&eth), "an unassociated station cannot frame a packet");
}

#[test]
fn association_brings_the_link_up_and_publishes_the_mac() {
    let (mut link, _tx) = link_with(Card::new(0, 0), RxBuf::new(0x4000_0000));
    link.associate(AP_MAC, PTK);
    assert!(link.link_up(), "associated means link up");
    assert_eq!(link.mac(), Some(STA_MAC), "the station MAC is published once up");
}

#[test]
fn transmit_encrypts_the_packet_in_software_and_kicks_the_queue() {
    let card = Card::new(0, 0);
    let probe = card.clone();
    let (mut link, tx) = link_with(card, RxBuf::new(0x4000_0000));
    link.associate(AP_MAC, PTK);

    let eth = eth_frame(AP_MAC, STA_MAC, [0x08, 0x00], &[0xAB]);
    assert!(link.send_tx(&eth), "an associated station frames and enqueues");

    /* Descriptor word 1 bits 22..23: no radio cipher; the MPDU is already CCMP. */
    let mem = tx.mem.borrow();
    let w1 = u32::from_le_bytes([mem[4], mem[5], mem[6], mem[7]]);
    assert_eq!((w1 >> 22) & 0x3, 0, "the TX descriptor asks the radio for no cipher");
    assert_eq!(mem[TXDESC_LEN + 1] & 0x40, 0x40, "the MPDU is marked Protected");
    let body = &mem[TXDESC_LEN..TXDESC_LEN + 96];
    assert!(!body.windows(6).any(|w| w == [0xAA, 0xAA, 0x03, 0, 0, 0]), "no plaintext LLC");
    drop(mem);

    // The write-index doorbell was rung so the card sees the new descriptor.
    assert!(probe.wrote(REG_TXBD_IDX_BEQ), "the BE queue write index is kicked");
}
