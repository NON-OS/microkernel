// NONOS Operating System (AGPL-3.0-or-later)
//! A transmit ring the card stops reading is reset after its five seconds
//!: both rings reprogrammed and the DMA interface reset, the
//! queued frames dropped and the reset counted. A ring the card keeps reading,
//! or an empty one, is left alone.

use super::card::Card;
use super::rig::{link_with, AP_MAC, PTK};
use super::rxbuf::RxBuf;
use crate::tx::stall::{TxWatch, STALL_MS};

const REG_TXBD_RWPTR_CLR: usize = 0x039C;

#[test]
fn the_watch_starts_when_frames_wait_and_restarts_when_the_card_reads() {
    let mut w = TxWatch::default();
    assert!(!w.stalled(true, 3, 10_000), "first sight starts the clock");
    assert!(!w.stalled(true, 3, 10_000 + STALL_MS - 1));
    assert!(!w.stalled(true, 4, 10_000 + STALL_MS), "the card read one: again from now");
    assert!(!w.stalled(true, 4, 10_000 + 2 * STALL_MS - 1));
    assert!(w.stalled(true, 4, 10_000 + 2 * STALL_MS));
    assert!(!w.stalled(false, 4, 10_000 + 9 * STALL_MS), "nothing queued is never a stall");
    assert!(!w.stalled(true, 4, 10_000 + 9 * STALL_MS), "and the clock starts afresh");
}

#[test]
fn a_ring_the_card_stopped_reading_is_reset_and_counted() {
    let card = Card::new(0, 0);
    let (mut link, _tx) = link_with(card.clone(), RxBuf::new(0x4000_0000));
    link.associate(AP_MAC, PTK);
    let clears = || card.writes.borrow().iter().filter(|w| w.0 == REG_TXBD_RWPTR_CLR).count();
    let at_bring_up = clears();
    assert!(link.send_raw(&[0x08; 40]) && link.send_raw(&[0x08; 40]));
    link.tick(1_000);
    link.tick(1_000 + STALL_MS - 1);
    assert_eq!((link.stats().tx_resets, clears()), (0, at_bring_up), "not yet");
    link.tick(1_000 + STALL_MS);
    assert_eq!((link.stats().tx_resets, clears()), (1, at_bring_up + 1), "reset once");
    link.tick(1_000 + 3 * STALL_MS);
    assert_eq!(link.stats().tx_resets, 1, "an empty ring is not reset again");
}
