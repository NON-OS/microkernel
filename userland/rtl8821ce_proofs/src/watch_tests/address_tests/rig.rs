// NONOS Operating System (AGPL-3.0-or-later)

//! A link on a card that keeps every 32-bit write.

use std::cell::RefCell;
use std::rc::Rc;

use super::super::card::NoRx;
use super::super::rig::{Dma, STA};
use crate::link::RtlLink;
use crate::regs::Mmio;

/// The core proofs' vector: home under PSK from a key of 0x5A bytes.
pub(super) const HOME: [u8; 6] = [0x9E, 0x14, 0x48, 0x4E, 0x38, 0x0D];

/// A card that keeps every 32-bit write.
#[derive(Clone, Default)]
pub(super) struct Log(pub(super) Rc<RefCell<Vec<(usize, u32)>>>);

impl Mmio for Log {
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
    fn write32(&self, off: usize, v: u32) {
        self.0.borrow_mut().push((off, v));
    }
}

pub(super) fn link(log: &Log) -> RtlLink<Log, Dma, NoRx> {
    let dma = |len: usize, at: u64| Dma(Rc::new(RefCell::new(vec![0u8; len])), at);
    RtlLink::new(log.clone(), dma(4096, 0x1000), dma(1 << 16, 0x2000), dma(4096, 0x3000), NoRx, STA)
}
