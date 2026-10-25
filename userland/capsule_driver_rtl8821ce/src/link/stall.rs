// NONOS Operating System (AGPL-3.0-or-later)

//! The transmit ring watched, and both rings reset when the card stopped
//! reading it.

use super::buffers::RxBuffers;
use super::rings::program_rings;
use super::state::RtlLink;
use crate::fw::dma::DmaMem;
use crate::regs::Mmio;
use crate::rx::ring::{RxState, RX_DESC_COUNT};
use crate::tx::regs::{REG_TXBD_IDX_BEQ, TRX_BD_HW_IDX_MASK, TRX_BD_HW_IDX_SHIFT};
use crate::tx::ring::{TxState, TX_DESC_COUNT};
use crate::tx::stall::TxWatch;

impl<M: Mmio, D: DmaMem, RB: RxBuffers> RtlLink<M, D, RB> {
    /// Watch the transmit ring at `now_ms`; reset the rings once the card has
    /// read nothing of a queued frame for `tx::stall::STALL_MS`.
    pub(super) fn watch_tx(&mut self, now_ms: u64) {
        let idx = self.mmio.read32(REG_TXBD_IDX_BEQ);
        self.tx_state.set_hw_rp((idx & TRX_BD_HW_IDX_MASK) >> TRX_BD_HW_IDX_SHIFT);
        let pending = self.tx_state.rp != self.tx_state.wp;
        if self.tx_watch.stalled(pending, self.tx_state.rp, now_ms) {
            self.reset_rings();
        }
    }

    /// Both rings programmed and the DMA interface reset as at bring-up
    /// (`new`), their indices from zero. The frames still queued are dropped:
    /// the card would never have sent them.
    pub(super) fn reset_rings(&mut self) {
        program_rings(&self.mmio, &self.tx_ring, &self.rx_ring, self.rx_buffers.device_addr());
        self.tx_state = TxState::new(TX_DESC_COUNT);
        self.rx_state = RxState::new(RX_DESC_COUNT);
        self.tx_watch = TxWatch::default();
        self.stats.tx_resets = self.stats.tx_resets.wrapping_add(1);
    }
}
