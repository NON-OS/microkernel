// NONOS Operating System (AGPL-3.0-or-later)

//! A transmit slot keeps no byte of the frame it held before.

use core::cell::RefCell;

use crate::fw::dma::DmaMem;
use crate::regs::Mmio;
use crate::tx::desc::FrameMeta;
use crate::tx::enqueue;
use crate::tx::regs::QSEL_BE;
use crate::tx::ring::{TxState, TX_BUF_STRIDE, TX_DESC_COUNT};

struct Mem(RefCell<Vec<u8>>);

impl DmaMem for Mem {
    fn capacity(&self) -> usize {
        self.0.borrow().len()
    }
    fn device_addr(&self) -> u64 {
        0x5000_0000
    }
    fn write_bytes(&self, offset: usize, src: &[u8]) {
        self.0.borrow_mut()[offset..offset + src.len()].copy_from_slice(src);
    }
}

/// Every register reads zero: the ring is empty.
struct Card;

impl Mmio for Card {
    fn read8(&self, _: usize) -> u8 {
        0
    }
    fn write8(&self, _: usize, _: u8) {}
    fn read16(&self, _: usize) -> u16 {
        0
    }
    fn write16(&self, _: usize, _: u16) {}
    fn read32(&self, _: usize) -> u32 {
        0
    }
    fn write32(&self, _: usize, _: u32) {}
}

#[test]
fn a_short_frame_clears_what_a_longer_one_left_in_its_slot() {
    let n = TX_DESC_COUNT as usize;
    let ring = Mem(RefCell::new(vec![0; n * 16]));
    // Every slot still holds an earlier frame's bytes.
    let buffers = Mem(RefCell::new(vec![0xEE; n * TX_BUF_STRIDE]));
    let meta = FrameMeta { qsel: QSEL_BE, bmc: false, rate: None, seq: 1, sec_type: 0 };
    let mut state = TxState::new(TX_DESC_COUNT);
    assert!(enqueue(&Card, &ring, &buffers, &mut state, &[0xC3; 10], &meta));
    let b = buffers.0.borrow();
    assert_eq!(&b[48..58], &[0xC3; 10], "the frame after its descriptor");
    assert!(b[58..TX_BUF_STRIDE].iter().all(|&x| x == 0), "the rest of the slot cleared");
    assert!(b[TX_BUF_STRIDE..].iter().all(|&x| x == 0xEE), "no other slot touched");
}
