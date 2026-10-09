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
//! 4.6.1.2, Linux `xhci_abort_cmd_ring`). A command that never completes
//! leaves the ring running on it, and every command queued after it waits
//! behind it for good. Writing CRCR.CA makes the controller give the command
//! up with Command Aborted, stop the ring with Command Ring Stopped, and
//! drop CRR; the next doorbell restarts the ring at the TRB after the one
//! given up, which is where this driver enqueues next.

use nonos_libc::{mk_idle_ms, Deadline};

use super::state::CommandRing;
use crate::constants::CRCR_LO;
use crate::regs::{mmio_read32, mmio_write32};

const CRCR_CA: u32 = 1 << 2;
const CRCR_CRR: u32 = 1 << 3;
/// Linux allows the abort five seconds.
const ABORT_TIMEOUT_MS: u64 = 5_000;
const ABORT_POLL_MS: u64 = 1;

impl CommandRing {
    pub(crate) fn set_op_base(&mut self, op_base: u64) {
        self.op_base = op_base;
    }

    /// Abort the running command. Returns whether the ring stopped. A ring
    /// never programmed into a controller has nothing to abort.
    pub fn abort(&self) -> bool {
        if self.op_base == 0 {
            return false;
        }
        let crcr = self.op_base + CRCR_LO;
        if mmio_read32(crcr) & CRCR_CRR == 0 {
            return true;
        }
        // While the ring runs the pointer bits are ignored; only CA acts. Only
        // the low dword is written: Linux found that a following write of
        // the high dword, landing after the ring stopped, is taken as a new
        // ring pointer and corrupts it.
        mmio_write32(crcr, CRCR_CA);
        let deadline = Deadline::after_ms(ABORT_TIMEOUT_MS);
        loop {
            if mmio_read32(crcr) & CRCR_CRR == 0 {
                return true;
            }
            if deadline.expired() {
                return false;
            }
            let _ = mk_idle_ms(ABORT_POLL_MS);
        }
    }
}
