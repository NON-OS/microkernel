// NONOS Operating System (AGPL-3.0-or-later)
//! The drive ending what it holds: a command, with its data and status, or
//! a busy spell, with its ready status; or dropping a command as it resets
//! itself. The HBA moves a drive's data through slot 0 as slot 0 reads at
//! that moment, whichever command the drive is ending.

use std::time::{Duration, Instant};

use super::fetch;
use super::hba::Hba;
use super::script::{Done, Outcome};
use super::state::{DIAG_N, DIAG_X};

const IS_DHRS: u32 = 1 << 0;
const IS_TFES: u32 = 1 << 30;

impl Hba {
    pub(super) fn complete(&mut self, done: Done) {
        if let Outcome::Error(err) = done.outcome {
            self.reg.tfd = u32::from(err) << 8 | 0x51;
            self.reg.is |= IS_TFES;
            return;
        }
        if self.running() && self.sent {
            let moved = self.drive.move_data(&done, &fetch::slot(self.clb));
            fetch::set_prdbc(self.clb, self.quirks.prdbc.unwrap_or(moved));
            (self.reg.ci, self.sent) = (self.reg.ci & !1, false);
        }
        self.reg.tfd = 0x50;
        self.reg.is |= IS_DHRS;
    }

    /// The drive's ready status, a D2H Register FIS with BSY clear. The HBA
    /// takes any such FIS as the end of a non-queued command in slot 0, so
    /// a command FIS the drive lost while busy is ended, nothing moved.
    pub(super) fn ready(&mut self) {
        self.reg.tfd = 0x50;
        self.reg.is |= IS_DHRS;
        if self.running() && self.sent && self.drive.idle() {
            (self.reg.ci, self.sent) = (self.reg.ci & !1, false);
            self.count.unrun += 1;
        }
    }

    /// The drive resetting itself while it holds a command
    /// (`self_reset_ms`): the command dropped, BSY until its signature, and
    /// the PHY renegotiating, which the HBA latches in PxSERR.
    pub(super) fn reset_itself(&mut self, now: Instant) {
        let due = self.at.taken.zip(self.quirks.self_reset_ms);
        if self.drive.idle() || due.is_none_or(|(t, ms)| now < t + Duration::from_millis(ms)) {
            return;
        }
        self.quirks.self_reset_ms = None;
        self.drive.reset();
        (self.reg.tfd, self.reg.ssts, self.reg.serr) = (0x80, 0x1, self.reg.serr | DIAG_N | DIAG_X);
        self.at.link = Some(now + Duration::from_millis(self.quirks.link_ms));
    }
}
