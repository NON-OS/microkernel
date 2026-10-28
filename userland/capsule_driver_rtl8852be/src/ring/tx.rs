// NONOS Operating System (AGPL-3.0-or-later)

//! A transmit ring: `len` buffer descriptors at the head of one DMA region
//! and one buffer slot per descriptor after them. The host write pointer
//! goes to the index register (`__rtw89_pci_tx_kick_off`); the card's read
//! pointer comes back in its high half (`rtw89_pci_txbd_recalc`); one
//! descriptor stays empty so a full ring is told from an empty one
//! (`rtw89_pci_get_avail_txbd_num`).

use super::addrs::ChAddrs;
use super::bd::{tx_bd, TX_BD_SIZE};
use crate::dma::DmaMem;
use crate::regs::Mmio;

/// Bytes of one buffer slot: a WD page and a frame, or an H2C packet.
pub const SLOT: usize = 4096;

pub struct TxRing<M: DmaMem> {
    pub mem: M,
    pub ch: ChAddrs,
    pub len: u32,
    pub wp: u32,
    pub rp: u32,
    /// Frame slots whose release report has not come (`tx::wd_ring`).
    pub busy: u64,
}

impl<M: DmaMem> TxRing<M> {
    /// A ring of `len` descriptors over `mem`, which must hold the
    /// descriptors' page and `len` slots (`bytes_for`).
    pub fn new(mem: M, ch: ChAddrs, len: u32) -> Option<Self> {
        let fits = len as usize <= SLOT / TX_BD_SIZE && len <= 64;
        let ok = len >= 2 && fits && mem.capacity() >= bytes_for(len);
        ok.then_some(Self { mem, ch, len, wp: 0, rp: 0, busy: 0 })
    }

    /// The slot's offset in the region.
    pub fn slot_off(&self, i: u32) -> usize {
        SLOT + i as usize * SLOT
    }

    /// Descriptors free to fill.
    pub fn avail(&self) -> u32 {
        if self.rp > self.wp {
            self.rp - self.wp - 1
        } else {
            self.len - (self.wp - self.rp) - 1
        }
    }

    /// Point the next descriptor at `len` bytes of its own slot and step the
    /// write pointer (`rtw89_pci_tx_bd_ring_update`). False when full.
    pub fn post(&mut self, len: u16) -> bool {
        let Some(dma) = self.mem.bus_at(self.slot_off(self.wp)) else { return false };
        if self.avail() == 0 || len as usize > SLOT {
            return false;
        }
        self.mem.write(self.wp as usize * TX_BD_SIZE, &tx_bd(len, dma));
        self.wp = (self.wp + 1) % self.len;
        true
    }

    /// Hand the card everything posted: the write pointer to the index.
    pub fn kick(&self, r: &impl Mmio) {
        r.write16(self.ch.idx, self.wp as u16);
    }
}

/// The region a ring of `len` descriptors needs: a page of descriptors, then
/// a slot per descriptor.
pub const fn bytes_for(len: u32) -> usize {
    SLOT + len as usize * SLOT
}
