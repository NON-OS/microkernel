// NONOS Operating System (AGPL-3.0-or-later)
//! A port that changes once, after a set number of reads, and the waits
//! run on it.

use crate::constants::regs::{CMD_CR, CMD_ST, PORT_CI, PORT_CMD, PORT_IS, PORT_SACT, PORT_TFD};
use crate::engine::completion::wait_done;
use crate::error::AhciError;

pub(crate) const LIMIT: u32 = 1_000;

/// PxCMD of a port whose command engine runs.
pub(crate) const RUNNING_ENGINE: u32 = CMD_ST | CMD_CR;

/// An `expired` for the waits that says the time is up on its `n`th call:
/// the wait then gives up after `n` looks, so a test counts reads, not time.
pub(crate) fn rounds_of(n: u32) -> impl FnMut() -> bool {
    let mut asked = 0u32;
    move || {
        asked += 1;
        asked >= n
    }
}

/// What the port registers the wait reads hold at one moment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Snap {
    pub ci: u32,
    pub sact: u32,
    pub is: u32,
    pub tfd: u32,
}

/// Slot 0 issued, the device busy on it: the HBA set BSY when it sent the
/// command FIS.
pub(crate) const RUNNING: Snap = Snap { ci: 1, sact: 0, is: 0, tfd: 0x80 };
/// Slot 0 issued as QEMU shows it: PxTFD changes only when a FIS arrives,
/// so while the command runs it still holds the last command's idle status.
pub(crate) const RUNNING_STALE: Snap = Snap { ci: 1, sact: 0, is: 0, tfd: 0x50 };
/// Slot 0 clear, a D2H FIS with DRDY and DSC posted (DHRS in PxIS).
pub(crate) const CLEAN_DONE: Snap = Snap { ci: 0, sact: 0, is: 1, tfd: 0x50 };

/// A port that shows `before` until `done_after` reads have been made, and
/// `after` from then on, its engine as `cmd` says. Every read is logged.
pub(crate) struct ModelPort {
    pub before: Snap,
    pub after: Snap,
    pub done_after: usize,
    pub cmd: u32,
    pub log: Vec<(u32, u32)>,
}

impl ModelPort {
    pub fn new(before: Snap, after: Snap, done_after: usize) -> Self {
        Self { before, after, done_after, cmd: RUNNING_ENGINE, log: Vec::new() }
    }

    pub fn read(&mut self, off: u32) -> u32 {
        let s = if self.log.len() >= self.done_after { self.after } else { self.before };
        let v = match off {
            PORT_CMD => self.cmd,
            PORT_CI => s.ci,
            PORT_SACT => s.sact,
            PORT_IS => s.is,
            PORT_TFD => s.tfd,
            _ => panic!("the wait read a register it has no business with: {off:#x}"),
        };
        self.log.push((off, v));
        v
    }
}

pub(crate) fn done(before: Snap, after: Snap, done_after: usize) -> Result<(), AhciError> {
    let mut port = ModelPort::new(before, after, done_after);
    wait_done(|off| port.read(off), rounds_of(LIMIT))
}
