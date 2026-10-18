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

//! A STALL racing the recovery of a request its device NAKs (Linux's
//! "Stop ep completion raced with stall"): the device STALLs just after a
//! Reset Endpoint found EP0 Running, so EP0 is Halted by the time the Stop
//! Endpoint that follows runs, as its endpoint context then says.

use nonos_libc::dma_host;

use super::hc::Hc;
use crate::event_ring::events::{transfer, CC_STALL};

pub(super) const CONTEXT_STATE_ERROR: u8 = 19;
const HALTED: u32 = 2;
pub(super) const STOPPED: u32 = 3;

#[derive(Default)]
pub struct Race {
    /// Every IN data stage is NAKed: it never completes, and EP0 runs on.
    pub nak_in: bool,
    /// EP0's endpoint context, by bus address, in the output context of
    /// the slot whose device STALLs once a Reset Endpoint finds it Running.
    pub halt_at: Option<u64>,
    /// The slot that STALL halted, and that context, until it is reset.
    pub halted: Option<(u8, u64)>,
    /// The controller fails, a Host Controller Error, as an EP0 doorbell
    /// rings, and runs the TD all the same.
    pub fail_at_ep0: bool,
}

impl Hc {
    /// The Reset Endpoint that found `slot`'s EP0 Running, and the STALL
    /// that follows it: Context State Error, EP0 Halted.
    pub(super) fn stall_after(&mut self, slot: u8, at: u64) -> u8 {
        (self.race.halted, self.race.nak_in) = (Some((slot, at)), false);
        ep_state(at, HALTED);
        self.producer.post(transfer(self.ep0[slot as usize].0, CC_STALL, 0, slot, 1));
        CONTEXT_STATE_ERROR
    }
}

/// Write `state` as the EP State of the endpoint context at bus address `at`.
pub(super) fn ep_state(at: u64, state: u32) {
    if let Some(host) = dma_host(at) {
        // SAFETY: `dma_host` named the live output context the test gave.
        unsafe { (host as *mut u32).write_volatile(state) };
    }
}
