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

//! The verb transport: one command out through the CORB, its answer back
//! through the RIRB.
//!
//! The driver used to read each answer from the RIRB slot with the same
//! number as the CORB slot it had posted to. The two rings advance
//! independently: the RIRB also carries unsolicited responses (a jack
//! plugged in, once a pin is told to report), so after the first one every
//! answer was read from the wrong slot. Here the driver keeps its own RIRB
//! read pointer, as Linux does (`snd_hdac_bus_update_rirb`), consumes every
//! entry the controller has written, sets unsolicited ones aside by the
//! UNSOL bit of the entry's upper word, and takes the first solicited answer
//! from the codec that was asked.
//!
//! A command that gets no answer still moved the controller's write pointer,
//! so the shadow moves with it. Left behind, the next command went into the
//! same slot under the same CORBWP, the controller saw nothing new, and every
//! verb after one slow answer timed out.

use core::ptr::{read_volatile, write_volatile};

use super::dma_sync::flush;
use super::wait::Wait;
use crate::constants::{
    CORBWP, RIRBSTS, RIRBSTS_INTFL, RIRBWP, RIRB_EX_CODEC, RIRB_EX_UNSOL,
};
use crate::error::{HdaError, HdaResult};
use crate::regs::Regs;

/// Linux waits up to a second for an answer; a working codec answers in
/// microseconds, and a setup that waits a second per verb on a dead one
/// takes minutes to give up. 100 ms covers the slowest power transition a
/// codec acknowledges before it completes.
pub const VERB_MS: u64 = 100;

pub struct Link {
    regs: Regs,
    corb_va: u64,
    rirb_va: u64,
    mask: u16,
    wp: u16,
    rp: u16,
    unsolicited: u32,
}

impl Link {
    /// A link over rings of `entries` slots, both pointers at zero, as
    /// `corb::init` leaves them.
    pub fn new(regs: Regs, corb_va: u64, rirb_va: u64, entries: u16) -> Self {
        let mask = if entries == 0 { 0xff } else { entries - 1 };
        Link { regs, corb_va, rirb_va, mask, wp: 0, rp: 0, unsolicited: 0 }
    }

    /// The controller register window, for the Immediate Command fallback when
    /// a verb gets no answer over the ring.
    pub fn regs(&self) -> Regs {
        self.regs
    }

    /// Unsolicited responses seen and set aside. A codec sends one when a
    /// pin's jack changes, so a new one is a reason to read the jack now.
    pub fn unsolicited(&self) -> u32 {
        self.unsolicited
    }

    /// Post `cmd` and wait for the answer from the codec it addresses.
    pub fn send(&mut self, cmd: u32) -> HdaResult<u32> {
        let cad = (cmd >> 28) as u8;
        // Anything already in the RIRB answers a command given up on.
        let _ = self.drain(None);
        let next = (self.wp + 1) & self.mask;
        unsafe {
            let slot = self.corb_va + next as u64 * 4;
            write_volatile(slot as *mut u32, cmd);
            flush(slot, 4);
            self.regs.w16(CORBWP, next);
        }
        self.wp = next;
        let mut w = Wait::ms(VERB_MS);
        loop {
            match self.drain(Some(cad)) {
                Some(resp) => {
                    unsafe { self.regs.w8(RIRBSTS, RIRBSTS_INTFL) };
                    return Ok(resp);
                }
                None if w.expired() => return Err(HdaError::VerbTimeout),
                None => core::hint::spin_loop(),
            }
        }
    }

    /// Consume every RIRB entry the controller has written. The first
    /// solicited answer from `cad` is returned; once it is found the rest
    /// stay for the next call.
    fn drain(&mut self, cad: Option<u8>) -> Option<u32> {
        let hw = unsafe { self.regs.r16(RIRBWP) };
        if hw == 0xffff {
            // A controller that dropped off the bus reads all ones.
            return None;
        }
        let hw = hw & self.mask;
        while self.rp != hw {
            self.rp = (self.rp + 1) & self.mask;
            let at = self.rirb_va + self.rp as u64 * 8;
            flush(at, 8);
            let (resp, ex) = unsafe {
                (read_volatile(at as *const u32), read_volatile((at + 4) as *const u32))
            };
            if ex & RIRB_EX_UNSOL != 0 {
                self.unsolicited = self.unsolicited.saturating_add(1);
                continue;
            }
            if let Some(c) = cad {
                if (ex & RIRB_EX_CODEC) as u8 == c {
                    return Some(resp);
                }
            }
        }
        None
    }
}
