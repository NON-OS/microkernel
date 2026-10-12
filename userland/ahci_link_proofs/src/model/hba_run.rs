// NONOS Operating System (AGPL-3.0-or-later)
//! What happens between two register accesses: the link coming back after a
//! COMRESET, the drive coming out of a busy spell or ending the command it
//! holds (`hba_end`), and the HBA sending slot 0's command FIS.

use std::time::{Duration, Instant};

use super::fetch;
use super::hba::Hba;
use super::state::{DIAG_N, LINK_UP};
use crate::constants::regs::{CMD_CR, CMD_ST, TFD_BSY, TFD_DRQ};

impl Hba {
    pub(super) fn running(&self) -> bool {
        self.reg.cmd & (CMD_ST | CMD_CR) == CMD_ST | CMD_CR
    }

    pub(super) fn advance(&mut self) {
        let now = crate::clock::now();
        if let Some(t) = self.quirks.busy_until.take() {
            (self.at.ready, self.reg.tfd) = (self.at.ready.max(Some(t)), self.reg.tfd | TFD_BSY);
        }
        self.reset_itself(now);
        if self.at.link.is_some_and(|t| now >= t) {
            // The drive posts its signature out of the reset: now, or
            // `reset_ms` on, or once a busy spell ends.
            self.at.link = None;
            let reset = now + Duration::from_millis(self.quirks.reset_ms.unwrap_or(0));
            self.at.ready = self.at.ready.max(Some(reset));
            (self.reg.ssts, self.reg.serr) = (LINK_UP, self.reg.serr | DIAG_N);
        }
        if self.at.link.is_none() && self.at.ready.is_some_and(|t| now >= t) {
            self.at.ready = None;
            self.ready();
        }
        if let Some(done) = self.drive.finished(now) {
            self.complete(done);
        }
        self.send(now);
    }

    fn send(&mut self, now: Instant) {
        let idle = self.reg.tfd & (TFD_BSY | TFD_DRQ) == 0 && self.at.link.is_none();
        if self.reg.ci & 1 == 0 || self.sent || !self.running() || !idle {
            return;
        }
        self.sent = true;
        if self.quirks.bsy_on_send {
            self.reg.tfd = 0x80;
        }
        self.reg.is |= self.quirks.fatal_on_send.take().unwrap_or(0);
        // A drive still busy, or without a link (DET not 3), never takes it.
        if self.at.ready.is_some() || self.reg.ssts & 0xf != 3 {
            self.count.lost += 1;
            return;
        }
        self.at.taken = Some(now);
        self.drive.take(fetch::command(self.clb), now);
    }
}
