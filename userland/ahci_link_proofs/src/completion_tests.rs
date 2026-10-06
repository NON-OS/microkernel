// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The driver's wait on a command, run against a model port. The device is
//! the adversary here: it may post any status, and it may finish between any
//! two register reads the driver makes.

use crate::constants::regs::{PORT_CI, PORT_IS, PORT_SACT, PORT_TFD};
use crate::engine::completion::{wait_done, wait_ready};
use crate::error::AhciError;

pub(crate) const LIMIT: u32 = 1_000;

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
/// `after` from then on. Every read is logged, in order.
pub(crate) struct ModelPort {
    pub before: Snap,
    pub after: Snap,
    pub done_after: usize,
    pub log: Vec<(u32, u32)>,
}

impl ModelPort {
    pub fn new(before: Snap, after: Snap, done_after: usize) -> Self {
        Self { before, after, done_after, log: Vec::new() }
    }

    pub fn read(&mut self, off: u32) -> u32 {
        let s = if self.log.len() >= self.done_after { self.after } else { self.before };
        let v = match off {
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

fn done(before: Snap, after: Snap, done_after: usize) -> Result<(), AhciError> {
    let mut port = ModelPort::new(before, after, done_after);
    wait_done(|off| port.read(off), rounds_of(LIMIT))
}

#[test]
fn a_clean_completion_is_done_wherever_it_lands() {
    for done_after in 0..24 {
        assert_eq!(done(RUNNING, CLEAN_DONE, done_after), Ok(()), "done after {done_after} reads");
    }
}

#[test]
fn a_command_the_device_never_finishes_times_out() {
    assert_eq!(done(RUNNING, RUNNING, usize::MAX), Err(AhciError::Timeout));
}

#[test]
fn a_task_file_error_fails_the_command() {
    /*
     * On an error the HBA leaves the slot's PxCI bit set and posts the
     * status with ERR (AHCI 1.3.1, 6.2.2).
     */
    let failed = Snap { ci: 1, sact: 0, is: 1 << 30, tfd: 0x0451 };
    for done_after in 0..24 {
        assert_eq!(done(RUNNING, failed, done_after), Err(AhciError::CommandFailed));
    }
    let err_only = Snap { ci: 1, sact: 0, is: 0, tfd: 0x51 };
    assert_eq!(done(RUNNING, err_only, 5), Err(AhciError::CommandFailed));
}

#[test]
fn each_fatal_interrupt_status_fails_the_command() {
    for bit in [27u32, 28, 29, 30] {
        let fatal = Snap { ci: 1, sact: 0, is: 1 << bit, tfd: 0x80 };
        assert_eq!(done(RUNNING, fatal, 3), Err(AhciError::CommandFailed), "PxIS bit {bit}");
    }
}

#[test]
fn a_device_stuck_busy_is_never_issued_to() {
    for tfd in [0x80u32, 0x08, 0x88, 0xff] {
        let busy = Snap { ci: 0, sact: 0, is: 0, tfd };
        let mut port = ModelPort::new(busy, busy, usize::MAX);
        assert_eq!(wait_ready(|off| port.read(off), rounds_of(LIMIT)), Err(AhciError::Timeout), "{tfd:#x}");
    }
}

#[test]
fn a_device_that_settles_is_issued_to() {
    let busy = Snap { ci: 0, sact: 0, is: 0, tfd: 0x80 };
    let idle = Snap { ci: 0, sact: 0, is: 0, tfd: 0x50 };
    for done_after in 0..8 {
        let mut port = ModelPort::new(busy, idle, done_after);
        assert_eq!(wait_ready(|off| port.read(off), rounds_of(LIMIT)), Ok(()), "idle after {done_after}");
    }
}
