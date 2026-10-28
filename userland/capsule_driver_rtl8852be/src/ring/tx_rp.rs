// NONOS Operating System (AGPL-3.0-or-later)

//! A transmit ring's read pointer, as the card reports it in the high half
//! of the ring's index register (`rtw89_pci_txbd_recalc`, Linux v6.12
//! rtw89 `pci.c`).

use super::addrs::hw_idx;
use super::tx::TxRing;
use crate::dma::DmaMem;
use crate::regs::Mmio;

impl<M: DmaMem> TxRing<M> {
    /// Read the card's read pointer back. A dead read (all ones) or a pointer
    /// past the ring is refused and the old one kept.
    pub fn reclaim(&mut self, r: &impl Mmio) -> bool {
        let v = r.read32(self.ch.idx);
        let rp = hw_idx(v);
        if v == u32::MAX || rp >= self.len {
            return false;
        }
        self.rp = rp;
        true
    }
}
