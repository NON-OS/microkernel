// NONOS Operating System (AGPL-3.0-or-later)

//! A rekeyed pairwise key, held until the CAM has it.

use super::buffers::RxBuffers;
use super::state::RtlLink;
use crate::fw::dma::DmaMem;
use crate::regs::Mmio;

impl<M: Mmio, D: DmaMem, RB: RxBuffers> RtlLink<M, D, RB> {
    /// The pairwise key a rekey delivered, its message 4 already sent.
    pub fn pairwise_key(&self) -> Option<&[u8; 16]> {
        self.new_pairwise_key.as_ref().map(|k| &k.0)
    }

    /// The CAM has the pairwise key: wiped where it lies.
    pub fn clear_pairwise_key(&mut self) {
        self.new_pairwise_key = None;
    }
}
