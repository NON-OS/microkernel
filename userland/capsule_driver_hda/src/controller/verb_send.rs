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

//! Posting a verb and waiting for its answer.
//!
//! The wait is bounded by `VERB_MS` of elapsed time. It used to spin for all
//! of it, and the jack is read twice a second, so a codec that stopped
//! answering held a core for a fifth of every second. A codec answers in
//! microseconds, so only the first millisecond or two spins; after that each
//! poll sleeps a millisecond and the core goes to the refill and the
//! requests (`wait`).

use core::ptr::write_volatile;

use super::dma_sync::flush;
use super::verb::{Link, VERB_MS};
use super::wait::Wait;
use crate::constants::{CORBWP, RIRBSTS, RIRBSTS_INTFL};
use crate::error::{HdaError, HdaResult};

impl Link {
    /// Post `cmd` and wait for the answer from the codec it addresses.
    pub fn send(&mut self, cmd: u32) -> HdaResult<u32> {
        let cad = (cmd >> 28) as u8;
        // Anything already in the RIRB answers a command given up on.
        let _ = self.drain(None);
        let next = (self.wp + 1) & self.mask;
        let slot = self.corb_va + next as u64 * 4;
        // SAFETY: `next` is masked to the ring, at most 256 entries of 4 bytes,
        // all inside the CORB grant; CORBWP lies in the register window.
        unsafe {
            write_volatile(slot as *mut u32, cmd);
            flush(slot, 4);
            self.regs.w16(CORBWP, next);
        }
        self.wp = next;
        let mut w = Wait::ms(VERB_MS);
        loop {
            match self.drain(Some(cad)) {
                Some(resp) => {
                    // SAFETY: RIRBSTS lies in the register window.
                    unsafe { self.regs.w8(RIRBSTS, RIRBSTS_INTFL) };
                    return Ok(resp);
                }
                None if w.expired() => return Err(HdaError::VerbTimeout),
                None => w.idle(),
            }
        }
    }
}
