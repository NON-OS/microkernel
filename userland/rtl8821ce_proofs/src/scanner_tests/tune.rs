// NONOS Operating System (AGPL-3.0-or-later)
//! Tuning the synthesizer, as the scan does on every channel, kicks no
//! transmit.

use super::super::channels::SCAN_CHANNELS;
use crate::phy::channel::{set_rf, Bw};
use crate::tests_all::linkport_tests::card::Card;
use crate::tx::regs::REG_TXBD_IDX_BEQ;

#[test]
fn tuning_every_scan_channel_kicks_no_transmit() {
    let card = Card::new(0, 0);
    SCAN_CHANNELS.iter().for_each(|&ch| set_rf(&card, ch, Bw::W20));
    assert!(!card.writes.borrow().is_empty() && !card.wrote(REG_TXBD_IDX_BEQ));
}
