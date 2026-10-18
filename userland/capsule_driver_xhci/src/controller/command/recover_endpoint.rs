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
//! (xHCI 1.2 sections 4.6.8, 4.6.9 and 4.6.10, as recalled, unverified).
//! A STALL, babble or transaction error
//! halts the endpoint, and a halted endpoint ignores its doorbell for good:
//! a HID device that stalls SET_IDLE, as many mice do, would otherwise lose
//! its control pipe, and one bad interrupt report its whole input. The
//! endpoint is first left Stopped (`to_stopped`): reset if Halted, stopped
//! if still Running (a transfer that timed out). Then the dequeue pointer
//! moves to where the next TRB goes, past every TRB that will never
//! complete, and the events parked for the endpoint are dropped: they
//! finish transfers that were given up, and the ring will put new TRBs at
//! their addresses.

use super::endpoint_at::EndpointAt;
use super::recovered::Recovered;
use super::run_command::run_command;
use super::to_stopped::to_stopped;
use crate::error::XhciResult;
use crate::rings::command::CommandRing;
use crate::rings::event::EventRing;
use crate::trb::commands::set_dequeue_command;

pub fn recover_endpoint(
    doorbell_base: u64,
    intr_base: u64,
    cmd_ring: &mut CommandRing,
    evt_ring: &mut EventRing,
    ep: EndpointAt,
) -> XhciResult<Recovered> {
    let how = to_stopped(doorbell_base, intr_base, cmd_ring, evt_ring, ep)?;
    let (ring, slot, dci) = (ep.ring, ep.slot, ep.dci);
    let dequeue = ring.enqueue_phys();
    let set = set_dequeue_command(slot, dci, dequeue, ring.cycle() != 0, cmd_ring.cycle() != 0);
    let moved = run_command(doorbell_base, intr_base, cmd_ring, evt_ring, set);
    evt_ring.forget_parked(slot, dci);
    moved?;
    ring.retire();
    Ok(how)
}
