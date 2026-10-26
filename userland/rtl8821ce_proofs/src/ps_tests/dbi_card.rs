// NONOS Operating System (AGPL-3.0-or-later)
//! A card whose DBI window holds the PCIe link configuration byte (0x719):
//! a read lands it in byte 1 of the read data, a write takes byte 1 of the
//! write data; every transaction completes at once.

use std::cell::Cell;

use crate::regs::Mmio;

const WDATA: usize = 0x03E8;
const RDATA: usize = 0x03EC;
const FLAG: usize = 0x03F0;

#[derive(Default)]
pub(super) struct DbiCard {
    pub(super) link_cfg: Cell<u8>,
    pending: Cell<u8>,
    pub(super) writes: Cell<u32>,
}

impl Mmio for DbiCard {
    fn read8(&self, off: usize) -> u8 {
        if off == RDATA + 1 {
            self.link_cfg.get()
        } else {
            0
        }
    }
    fn write8(&self, off: usize, val: u8) {
        if off == WDATA + 1 {
            self.pending.set(val);
        } else if off == FLAG + 2 && val == 1 {
            self.link_cfg.set(self.pending.get());
            self.writes.set(self.writes.get() + 1);
        }
    }
    fn read16(&self, _: usize) -> u16 {
        0
    }
    fn write16(&self, _: usize, _: u16) {}
    fn read32(&self, _: usize) -> u32 {
        0
    }
    fn write32(&self, _: usize, _: u32) {}
}
