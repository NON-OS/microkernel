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

//! Aborting the command the controller is stuck on (xHCI 1.2 section
//! 4.6.1.2). A command that never completes
//! leaves the ring running on it, and every command queued after it waits
//! behind it for good. Writing CRCR.CA stops the ring and drops CRR. A
//! command the controller was running is given up with Command Aborted, and
//! Command Ring Stopped then names the TRB after it, where the next
//! doorbell restarts the ring. One it had not yet taken up gets no Command
//! Aborted: Command Ring Stopped names that TRB itself, and the next
//! doorbell would run it, so the caller makes it a No Op (`give_up`). The
//! events are the caller's to consume (`settle_abort`). A controller that
//! failed keeps CRR set, so it is not waited on.

use nonos_libc::{mk_idle_ms, Deadline};

use super::abort_outcome::Abort;
use super::controller::running;
use super::state::CommandRing;
use crate::constants::CRCR_LO;
use crate::regs::mmio_write32;
use crate::regs::op::controller_failed;

const CRCR_CA: u32 = 1 << 2;
/// Five seconds for the abort (as observed, unverified).
const ABORT_TIMEOUT_MS: u64 = 5_000;
const ABORT_POLL_MS: u64 = 1;

impl CommandRing {
    /// Abort the running command. A ring never programmed into a controller
    /// has nothing to abort and is not running.
    pub fn abort(&mut self) -> Abort {
        if self.op_base == 0 {
            return Abort::NotRunning;
        }
        let crcr = self.op_base + CRCR_LO;
        if !running(crcr) {
            return Abort::NotRunning;
        }
        // While the ring runs the pointer bits are ignored; only CA acts. Only
        // the low dword is written: a following write of
        // the high dword, landing after the ring stopped, is taken as a new
        // ring pointer and corrupts it (as observed, unverified).
        // SAFETY: as in `running`.
        unsafe { mmio_write32(crcr, CRCR_CA) };
        let deadline = Deadline::after_ms(ABORT_TIMEOUT_MS);
        loop {
            let failed = controller_failed(self.op_base);
            if !failed && !running(crcr) {
                return Abort::Stopped;
            }
            if failed || deadline.expired() {
                self.wedged = true;
                return Abort::Stuck;
            }
            let _ = mk_idle_ms(ABORT_POLL_MS);
        }
    }
}
