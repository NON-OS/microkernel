// NONOS Operating System (AGPL-3.0-or-later)

//! A card that left the bus reads back all ones: the link says it is gone,
//! and its receive index is not taken to name slots the card wrote.

use std::cell::RefCell;
use std::rc::Rc;

use super::card::Card;
use super::rig::{Dma, AP_MAC, PTK, STA_MAC};
use super::rxbuf::RxBuf;
use crate::link::RtlLink;
use crate::regs::Mmio;

// Every read all ones, as from a device that no longer answers.
#[derive(Clone, Default)]
struct Gone(Rc<RefCell<Vec<usize>>>);

impl Mmio for Gone {
    fn read8(&self, _: usize) -> u8 {
        u8::MAX
    }
    fn write8(&self, off: usize, _: u8) {
        self.0.borrow_mut().push(off);
    }
    fn read16(&self, _: usize) -> u16 {
        u16::MAX
    }
    fn write16(&self, off: usize, _: u16) {
        self.0.borrow_mut().push(off);
    }
    fn read32(&self, _: usize) -> u32 {
        u32::MAX
    }
    fn write32(&self, off: usize, _: u32) {
        self.0.borrow_mut().push(off);
    }
}

fn link<M: Mmio>(card: M, rx: RxBuf) -> RtlLink<M, Dma, RxBuf> {
    let (tx_ring, tx_buffers) = (Dma::new(1 << 12, 0x1000_0000), Dma::new(1 << 16, 0x2000_0000));
    RtlLink::new(card, tx_ring, tx_buffers, Dma::new(1 << 12, 0x3000_0000), rx, STA_MAC)
}

#[test]
fn a_card_reading_all_ones_is_gone_and_a_live_one_is_not() {
    assert!(link(Gone::default(), RxBuf::new(0x4000_0000)).card_gone());
    assert!(!link(Card::new(0, 0), RxBuf::new(0x4000_0000)).card_gone());
}

#[test]
fn an_all_ones_receive_index_hands_up_nothing() {
    let mut rx = RxBuf::new(0x4000_0000);
    rx.stage(0, &[0x08; 40]);
    let mut l = link(Gone::default(), rx);
    l.associate(AP_MAC, PTK);
    let mut out = [0u8; 2048];
    assert_eq!(l.poll_raw(&mut out), None, "no slot is read on a dead index");
}
