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

//! Bringing an endpoint back after a transfer on it failed or never ended
//! (xHCI 1.2 sections 4.6.8, 4.6.9 and 4.6.10, Linux
//! `xhci_handle_halted_endpoint`). A STALL, babble or transaction error
//! halts the endpoint, and a halted endpoint ignores its doorbell for good:
//! a HID device that stalls SET_IDLE, as many mice do, would otherwise lose
//! its control pipe, and one bad interrupt report its whole input. Reset
//! Endpoint moves a halted endpoint to Stopped; one still Running (a
//! transfer that timed out) answers Context State Error and is stopped
//! instead. Either way the dequeue pointer then moves to where the next TRB
//! goes, past every TRB that will never complete, and the events parked for
//! the endpoint are dropped: they finish transfers that were given up, and
//! the ring will put new TRBs at their addresses.

use super::run_command::run_command;
use crate::error::{XhciError, XhciResult};
use crate::rings::command::CommandRing;
use crate::rings::event::EventRing;
use crate::rings::transfer::TransferRing;
use crate::trb::commands::{reset_endpoint_command, set_dequeue_command, stop_endpoint_command};

/// The endpoint was not in the state the command needs.
pub const CC_CONTEXT_STATE_ERROR: u8 = 19;

/// How the endpoint was brought back.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Recovered {
    /// It was Halted, and Reset Endpoint also reset its data toggle or
    /// sequence number.
    Reset,
    /// It was Running or already Stopped: Stop Endpoint left the toggle
    /// where it was.
    Stopped,
}

pub fn recover_endpoint(
    doorbell_base: u64,
    intr_base: u64,
    cmd_ring: &mut CommandRing,
    evt_ring: &mut EventRing,
    ring: &TransferRing,
    slot: u8,
    dci: u8,
) -> XhciResult<Recovered> {
    let reset = reset_endpoint_command(slot, dci, cmd_ring.cycle() != 0);
    let how = match run_command(doorbell_base, intr_base, cmd_ring, evt_ring, reset) {
        Ok(_) => Recovered::Reset,
        Err(XhciError::CommandCompletionFailed(CC_CONTEXT_STATE_ERROR)) => {
            let stop = stop_endpoint_command(slot, dci, cmd_ring.cycle() != 0);
            match run_command(doorbell_base, intr_base, cmd_ring, evt_ring, stop) {
                Ok(_) | Err(XhciError::CommandCompletionFailed(CC_CONTEXT_STATE_ERROR)) => {}
                Err(e) => return Err(e),
            }
            Recovered::Stopped
        }
        Err(e) => return Err(e),
    };
    let dequeue = ring.enqueue_phys();
    let set = set_dequeue_command(slot, dci, dequeue, ring.cycle() != 0, cmd_ring.cycle() != 0);
    let moved = run_command(doorbell_base, intr_base, cmd_ring, evt_ring, set);
    evt_ring.forget_parked(slot, dci);
    moved.map(|_| how)
}
