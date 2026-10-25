// NONOS Operating System (AGPL-3.0-or-later)
//! the plan for the 8821C: Bus Master goes on after the power cycle stopped a
//! dead driver's engines, and before the first write that makes the card
//! fetch from memory: the beacon-queue poll of the first firmware chunk
//! (`fw::rsvd::stage_chunk`, PCI_BCNQ_FLAG in REG_TXBD_BCN_WORK). Writing
//! the ring's address before it fetches nothing.

use core::cell::RefCell;

use super::armed::{armed, Armed};
use crate::fw::dma::DmaMem;
use crate::fw::regs::{BCN_VALID_V1, PCI_BCNQ_FLAG, REG_FIFOPAGE_CTRL_2, REG_TXBD_BCN_WORK};
use crate::fw::rsvd::stage_chunk;
use crate::pwr::power_on_then_master;
use crate::regs::Mmio;

struct Buf(RefCell<Vec<u8>>, u64);

impl DmaMem for Buf {
    fn capacity(&self) -> usize {
        self.0.borrow().len()
    }
    fn device_addr(&self) -> u64 {
        self.1
    }
    fn write_bytes(&self, at: usize, src: &[u8]) {
        self.0.borrow_mut()[at..at + src.len()].copy_from_slice(src);
    }
}

/// The armed card, fetching at the beacon poll and then landing the page.
struct Fetches<'a>(&'a Armed);

impl Mmio for Fetches<'_> {
    fn read8(&self, off: usize) -> u8 {
        self.0.read8(off)
    }
    fn write8(&self, off: usize, val: u8) {
        self.0.write8(off, val);
        if off == REG_TXBD_BCN_WORK && val & PCI_BCNQ_FLAG != 0 {
            self.0.log.borrow_mut().push("fetch");
            let valid = self.0.read16(REG_FIFOPAGE_CTRL_2) | BCN_VALID_V1;
            self.0.write16(REG_FIFOPAGE_CTRL_2, valid);
        }
    }
    fn read16(&self, off: usize) -> u16 {
        self.0.read16(off)
    }
    fn write16(&self, off: usize, val: u16) {
        // Beacon-valid is write-1-to-clear.
        let v = if off == REG_FIFOPAGE_CTRL_2 { self.0.read16(off) & !val } else { val };
        self.0.write16(off, v);
    }
    fn read32(&self, off: usize) -> u32 {
        self.0.read32(off)
    }
    fn write32(&self, off: usize, val: u32) {
        self.0.write32(off, val);
    }
}

#[test]
fn bus_master_is_on_before_the_beacon_poll_that_makes_the_card_fetch() {
    let a = armed(true, true);
    let card = Fetches(&a);
    power_on_then_master(&card, || {
        a.log.borrow_mut().push("bus_master");
        true
    })
    .expect("powered off, on, then Bus Master");
    let (stage, ring) =
        (Buf(RefCell::new(vec![0; 4096]), 0x1000), Buf(RefCell::new(vec![0; 64]), 0x2000));
    assert!(stage_chunk(&card, &stage, &ring, &[0x5A; 256]).is_some(), "the page landed");
    assert_eq!(*a.log.borrow(), ["powered_off", "bus_master", "fetch"]);
}
