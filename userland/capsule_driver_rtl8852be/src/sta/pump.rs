// NONOS Operating System (AGPL-3.0-or-later)

//! Taking what the card has written: release reports from RPQ freeing the
//! frame slots they name (`rtw89_pci_release_rpp`: the report's queue
//! selects the ring, its sequence the slot), and the next frame from RXQ.
//! Every buffer taken is handed back to the card at once, as
//! `rtw89_pci_rxbd_deliver` writes the host index after each pass.

use super::chip::Chip;
use crate::dma::DmaMem;
use crate::regs::Mmio;
use crate::ring::set::{RPQ_BUF, RXQ_BUF};
use crate::rx::frame::{take, Drop};
use crate::rx::rpp::reports;
use crate::tx::wd::QSEL_MGMT;
use alloc::vec::Vec;

/// What one RXQ buffer gave.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Got {
    Frame(usize, bool),
    Dropped(Drop),
}

impl<R: Mmio, M: DmaMem> Chip<R, M> {
    /// Free the slots of every frame the MAC has reported done.
    pub fn reap(&mut self, scratch: &mut Vec<u8>) -> usize {
        let Some(n) = self.rings.rpq.ready(&self.r) else { return 0 };
        scratch.resize(RPQ_BUF, 0);
        let mut total = 0;
        for _ in 0..n {
            let q = &mut self.rings.rpq;
            let at = q.buf_off(q.wp);
            q.mem.read(at, &mut scratch[..]);
            let (ach0, ch8) = (&mut self.rings.ach0, &mut self.rings.ch8);
            total += reports(scratch, |rp| match rp.qsel as u32 {
                QSEL_MGMT => ch8.release(rp.seq),
                0 => ach0.release(rp.seq),
                _ => {}
            });
            self.rings.rpq.advance();
        }
        self.rings.rpq.release(&self.r);
        total
    }

    /// The next RXQ buffer, copied into `out` when it holds a frame.
    pub fn next_rx(&mut self, scratch: &mut Vec<u8>, out: &mut [u8]) -> Option<Got> {
        let n = self.rings.rxq.ready(&self.r)?;
        if n == 0 {
            return None;
        }
        scratch.resize(RXQ_BUF, 0);
        let q = &mut self.rings.rxq;
        let at = q.buf_off(q.wp);
        q.mem.read(at, &mut scratch[..]);
        let got = match take(scratch, out) {
            Ok((len, dec)) => Got::Frame(len, dec),
            Err(e) => Got::Dropped(e),
        };
        q.advance();
        q.release(&self.r);
        Some(got)
    }
}
