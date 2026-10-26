// NONOS Operating System (AGPL-3.0-or-later)
//! A card that takes reserved pages: the beacon-valid bit up once a staged
//! page landed (or never), its mailboxes free, every write kept; and staging
//! memory that keeps what it was given.

use std::cell::{Cell, RefCell};

use crate::fw::dma::DmaMem;
use crate::regs::Mmio;

pub(super) struct JoinCard {
    pub(super) lands: bool,
    aid_reg: Cell<u32>,
    pub(super) writes: RefCell<Vec<(usize, u32)>>,
}

impl JoinCard {
    pub(super) fn new(lands: bool) -> Self {
        Self { lands, aid_reg: Cell::new(0xF000_0000), writes: RefCell::default() }
    }
    pub(super) fn at(&self, off: usize) -> Vec<(usize, u32)> {
        let w = self.writes.borrow();
        w.iter().enumerate().filter(|(_, x)| x.0 == off).map(|(i, x)| (i, x.1)).collect()
    }
}

impl Mmio for JoinCard {
    fn read8(&self, _: usize) -> u8 {
        0
    }
    fn write8(&self, off: usize, val: u8) {
        self.writes.borrow_mut().push((off, val.into()));
    }
    fn read16(&self, off: usize) -> u16 {
        if off == 0x0204 && self.lands {
            0x8000
        } else {
            0
        }
    }
    fn write16(&self, off: usize, val: u16) {
        self.writes.borrow_mut().push((off, val.into()));
    }
    fn read32(&self, off: usize) -> u32 {
        if off == 0x06A8 {
            self.aid_reg.get()
        } else {
            0
        }
    }
    fn write32(&self, off: usize, val: u32) {
        if off == 0x06A8 {
            self.aid_reg.set(val);
        }
        self.writes.borrow_mut().push((off, val));
    }
}

#[derive(Default)]
pub(super) struct Mem(pub(super) RefCell<Vec<u8>>);

impl DmaMem for Mem {
    fn capacity(&self) -> usize {
        0x2000
    }
    fn device_addr(&self) -> u64 {
        0x5000_0000
    }
    fn write_bytes(&self, offset: usize, src: &[u8]) {
        let mut m = self.0.borrow_mut();
        let len = m.len().max(offset + src.len());
        m.resize(len, 0);
        m[offset..offset + src.len()].copy_from_slice(src);
    }
}
