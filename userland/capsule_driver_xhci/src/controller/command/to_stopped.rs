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

//! Leaving an endpoint Stopped so its dequeue pointer can be moved (xHCI
//! 1.2 sections 4.6.8 and 4.6.9). Reset Endpoint moves a Halted endpoint
//! there; one still Running answers Context State Error and is stopped
//! instead. One that halts in between, a STALL racing the Stop, answers
//! Stop with Context State Error too: its context, read again, then says
//! Halted, and it is reset after all (as observed, unverified). Left
//! halted, Set TR Dequeue failed and the request's buffer stayed
//! kept until Disable Slot.

use super::endpoint_at::EndpointAt;
use super::recovered::{Recovered, CC_CONTEXT_STATE_ERROR};
use super::run_command::run_command;
use crate::error::{XhciError, XhciResult};
use crate::rings::command::CommandRing;
use crate::rings::event::EventRing;
use crate::trb::commands::{reset_endpoint_command, stop_endpoint_command};

const STATE_ERROR: XhciError = XhciError::CommandCompletionFailed(CC_CONTEXT_STATE_ERROR);

pub(super) fn to_stopped(
    doorbell_base: u64,
    intr_base: u64,
    cmd_ring: &mut CommandRing,
    evt_ring: &mut EventRing,
    ep: EndpointAt,
) -> XhciResult<Recovered> {
    match reset(doorbell_base, intr_base, cmd_ring, evt_ring, ep) {
        Err(STATE_ERROR) => {}
        done => return done,
    }
    let stop = stop_endpoint_command(ep.slot, ep.dci, cmd_ring.cycle() != 0);
    match run_command(doorbell_base, intr_base, cmd_ring, evt_ring, stop) {
        Err(STATE_ERROR) if ep.halted() => reset(doorbell_base, intr_base, cmd_ring, evt_ring, ep),
        Ok(_) | Err(STATE_ERROR) => Ok(Recovered::Stopped),
        Err(e) => Err(e),
    }
}

fn reset(
    doorbell_base: u64,
    intr_base: u64,
    cmd_ring: &mut CommandRing,
    evt_ring: &mut EventRing,
    ep: EndpointAt,
) -> XhciResult<Recovered> {
    let trb = reset_endpoint_command(ep.slot, ep.dci, cmd_ring.cycle() != 0);
    run_command(doorbell_base, intr_base, cmd_ring, evt_ring, trb).map(|_| Recovered::Reset)
}
