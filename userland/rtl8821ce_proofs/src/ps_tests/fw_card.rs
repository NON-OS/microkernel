// NONOS Operating System (AGPL-3.0-or-later)
//! A card for the power save firmware path: its mailboxes free or held, its
//! transmit configuration with the power management bit set until the
//! firmware has cleared it a number of reads later, or never.

use std::cell::{Cell, RefCell};

use crate::regs::Mmio;

const HMETFR: usize = 0x01CC;
pub(super) const TCR: usize = 0x0604;

pub(super) struct FwCard {
    pub(super) boxes_held: bool,
    /// Reads of the transmit configuration before the firmware clears the
    /// bit; `None`, never.
    pub(super) clears_after: Option<u32>,
    tcr: Cell<u32>,
    reads: Cell<u32>,
    pub(super) writes: RefCell<Vec<(usize, u32)>>,
}

impl FwCard {
    pub(super) fn new(boxes_held: bool, clears_after: Option<u32>) -> Self {
        let (tcr, reads, writes) = (Cell::new(0x80), Cell::new(0), RefCell::default());
        Self { boxes_held, clears_after, tcr, reads, writes }
    }
    pub(super) fn wrote(&self, off: usize) -> Vec<u32> {
        self.writes.borrow().iter().filter(|w| w.0 == off).map(|w| w.1).collect()
    }
    pub(super) fn at(&self, off: usize, val: u32) -> Option<usize> {
        self.writes.borrow().iter().position(|&w| w == (off, val))
    }
}

impl Mmio for FwCard {
    fn read8(&self, off: usize) -> u8 {
        if off == HMETFR && self.boxes_held {
            0xFF
        } else {
            0
        }
    }
    fn write8(&self, off: usize, val: u8) {
        self.writes.borrow_mut().push((off, val.into()));
    }
    fn read16(&self, _: usize) -> u16 {
        0
    }
    fn write16(&self, off: usize, val: u16) {
        self.writes.borrow_mut().push((off, val.into()));
    }
    fn read32(&self, off: usize) -> u32 {
        if off != TCR {
            return 0;
        }
        self.reads.set(self.reads.get() + 1);
        if self.clears_after.is_some_and(|n| self.reads.get() > n) {
            self.tcr.set(self.tcr.get() & !0x80);
        }
        self.tcr.get()
    }
    fn write32(&self, off: usize, val: u32) {
        if off == TCR {
            self.tcr.set(val);
        }
        self.writes.borrow_mut().push((off, val));
    }
}
