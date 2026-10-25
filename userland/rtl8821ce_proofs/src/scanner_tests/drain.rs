// NONOS Operating System (AGPL-3.0-or-later)
//! The receive step drains what a station might answer and sends nothing.

use super::super::state::Scanner;
use super::rig::{provoking, Dma, AP, STA};
use crate::link::RtlLink;
use crate::tests_all::linkport_tests::card::Card;
use crate::tests_all::linkport_tests::rxbuf::RxBuf;
use crate::tx::regs::REG_TXBD_IDX_BEQ;

#[test]
fn the_scanner_drains_what_a_station_might_answer_and_sends_nothing() {
    for associated in [false, true] {
        let frames = provoking();
        let mut rx = RxBuf::new(0x4000_0000);
        frames.iter().enumerate().for_each(|(i, f)| rx.stage(i as u32, f));
        let card = Card::new(0, frames.len() as u32);
        let (ring, bufs, rx_ring) =
            (Dma::new(1 << 12, 0x1000), Dma::new(1 << 16, 0x2000), Dma::new(1 << 12, 0x3000));
        let mut link = RtlLink::new(card.clone(), ring.clone(), bufs.clone(), rx_ring, rx, STA);
        if associated {
            link.associate(AP, [0x5A; 16]);
        }
        let mut s = Scanner::new();
        (0..3).for_each(|_| s.drain(&mut link, &card));
        assert_eq!((s.steps, s.raw, s.beacons), (3, frames.len() as u32, 1), "{associated}");
        assert_eq!(s.cache().count(), 1, "the beacon's network is listed");
        assert!(
            (s.tuned, s.ch_idx, s.hop_at_ms, s.sweeps) == (true, 0, 0, 0),
            "tuned, never hopped"
        );
        assert!(
            ring.0.borrow().iter().chain(bufs.0.borrow().iter()).all(|&b| b == 0),
            "nothing queued"
        );
        assert!(!card.wrote(REG_TXBD_IDX_BEQ), "the TX queue never kicked");
    }
}
