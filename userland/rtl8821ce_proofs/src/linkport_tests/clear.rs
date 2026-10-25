// NONOS Operating System (AGPL-3.0-or-later)

//! A receive slot goes back to the card holding nothing of what it carried:
//! a delivered frame's slot and a skipped one's alike, and only those.

use super::card::Card;
use super::receive::{chip_decrypted, W0_ENC_AES};
use super::rig::{eth_frame, Dma, AP_MAC, STA_MAC};
use super::rxbuf::RxBuf;
use crate::rx::poll_one_info;
use crate::rx::ring::{RxState, RX_BUF_STRIDE, RX_DESC_COUNT};

const W0_CRC32: u32 = 1 << 14;

#[test]
fn every_slot_taken_is_cleared_before_the_card_gets_it_back() {
    let f = chip_decrypted(&eth_frame(STA_MAC, AP_MAC, [0x08, 0x00], &[9; 40]), 1);
    let mut rx = RxBuf::new(0x4000_0000);
    rx.stage_w0(0, &f, W0_ENC_AES | W0_CRC32); // skipped: a CRC error
    rx.stage_w0(1, &f, W0_ENC_AES); // delivered
    rx.stage_w0(2, &f, W0_ENC_AES); // not yet ready
    let ring = Dma::new(4096, 0x1000_0000);
    let mut state = RxState::new(RX_DESC_COUNT);
    let mut out = [0u8; 1600];
    let got = poll_one_info(&Card::new(0, 2), &ring, &mut rx, &mut state, &mut out);
    assert!(got.is_some(), "slot 1 delivered");
    let slot = |i: usize| &rx.mem[i * RX_BUF_STRIDE..(i + 1) * RX_BUF_STRIDE];
    assert!(slot(0).iter().all(|&b| b == 0), "the skipped slot cleared");
    assert!(slot(1).iter().all(|&b| b == 0), "the delivered slot cleared");
    assert!(slot(2).iter().any(|&b| b != 0), "a slot not taken is left alone");
}
