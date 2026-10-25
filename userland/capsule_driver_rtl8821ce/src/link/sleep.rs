// NONOS Operating System (AGPL-3.0-or-later)

//! The link's power save: held with the link, watched on its clock, woken
//! before anything that needs the radio awake.

use super::buffers::RxBuffers;
use super::state::RtlLink;
use crate::fw::dma::DmaMem;
use crate::ps::{Ps, Seen};
use crate::regs::Mmio;

impl<M: Mmio, D: DmaMem, RB: RxBuffers> RtlLink<M, D, RB> {
    /// Power save as bring-up found the PCIe link.
    pub fn set_ps(&mut self, ps: Ps) {
        self.ps = ps;
    }

    /// For the proofs: the link's power save.
    #[cfg(test)]
    pub fn ps(&self) -> &Ps {
        &self.ps
    }

    /// Awake before a teardown, a join or a scan.
    pub fn wake_ps(&mut self) {
        self.ps.wake(&self.mmio);
    }

    pub(super) fn ps_tick(&mut self, now_ms: u64) {
        let s = self.stats;
        let seen = Seen { tx: s.tx_ok, rx: s.rx_eth, sends: s.watch_sends };
        let may_sleep = self.station.is_associated() && !self.station.busy();
        self.ps.tick(&self.mmio, now_ms, seen, may_sleep);
    }
}
