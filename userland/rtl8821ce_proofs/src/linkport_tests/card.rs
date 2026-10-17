// NONOS Operating System (AGPL-3.0-or-later)

//! The modeled card: ring indices, and every write kept.

use std::cell::RefCell;
use std::rc::Rc;

use crate::regs::Mmio;
use crate::rx::regs::REG_RXBD_IDX_MPDUQ;
use crate::tx::regs::{REG_TXBD_IDX_BEQ, TRX_BD_HW_IDX_SHIFT};

// A modeled card. Both ring index registers report a settable hardware pointer in
// their high bits; every write is recorded in a shared log so a test can assert
// the doorbell after the card has moved into the link.
#[derive(Clone)]
pub(crate) struct Card {
    pub(crate) tx_rp: u32,
    pub(crate) rx_wp: u32,
    pub(crate) writes: Rc<RefCell<Vec<(usize, u32)>>>,
}

impl Card {
    pub(crate) fn new(tx_rp: u32, rx_wp: u32) -> Self {
        Self { tx_rp, rx_wp, writes: Rc::new(RefCell::new(Vec::new())) }
    }
    pub(crate) fn wrote(&self, off: usize) -> bool {
        self.writes.borrow().iter().any(|&(o, _)| o == off)
    }
}

impl Mmio for Card {
    fn read8(&self, _o: usize) -> u8 {
        0
    }
    fn write8(&self, _o: usize, _v: u8) {}
    fn read16(&self, _o: usize) -> u16 {
        0
    }
    fn write16(&self, off: usize, val: u16) {
        self.writes.borrow_mut().push((off, val as u32));
    }
    fn read32(&self, off: usize) -> u32 {
        if off == REG_TXBD_IDX_BEQ {
            self.tx_rp << TRX_BD_HW_IDX_SHIFT
        } else if off == REG_RXBD_IDX_MPDUQ {
            self.rx_wp << TRX_BD_HW_IDX_SHIFT
        } else {
            0
        }
    }
    fn write32(&self, off: usize, val: u32) {
        self.writes.borrow_mut().push((off, val));
    }
}
