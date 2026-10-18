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
//! One command: queued, rung, and waited for. A command that times out is
//! given up so the commands after it are not stuck behind it on the ring,
//! and what giving it up leaves on the event ring is consumed here.
use super::give_up::give_up;
use super::next_completion::CommandCompletion;
use super::wait_command_completion::{wait_command_completion, wait_command_within};
use crate::controller::ring_doorbell::ring_doorbell;
use crate::error::{XhciError, XhciResult};
use crate::rings::command::CommandRing;
use crate::rings::event::EventRing;
use crate::trb::Trb;

pub fn run_command(
    doorbell_base: u64,
    intr_base: u64,
    cmd_ring: &mut CommandRing,
    evt_ring: &mut EventRing,
    trb: Trb,
) -> XhciResult<CommandCompletion> {
    let issued = cmd_ring.enqueue(trb)?;
    ring_doorbell(doorbell_base, 0, 0);
    let result = wait_command_completion(intr_base, issued, evt_ring);
    finish(intr_base, cmd_ring, evt_ring, issued, result)
}

/// The same, the command given up after `timeout_ms`.
pub fn run_command_within(
    doorbell_base: u64,
    intr_base: u64,
    cmd_ring: &mut CommandRing,
    evt_ring: &mut EventRing,
    trb: Trb,
    timeout_ms: u64,
) -> XhciResult<CommandCompletion> {
    let issued = cmd_ring.enqueue(trb)?;
    ring_doorbell(doorbell_base, 0, 0);
    let result = wait_command_within(intr_base, issued, evt_ring, timeout_ms);
    finish(intr_base, cmd_ring, evt_ring, issued, result)
}

/// A command answered, so run, leaves the ring with nothing the controller
/// has not taken; one that timed out is given up.
fn finish(
    intr_base: u64,
    cmd_ring: &mut CommandRing,
    evt_ring: &mut EventRing,
    issued: u64,
    result: XhciResult<CommandCompletion>,
) -> XhciResult<CommandCompletion> {
    match result {
        Err(XhciError::CommandCompletionTimeout) => give_up(intr_base, cmd_ring, evt_ring, issued),
        Ok(_) | Err(XhciError::CommandCompletionFailed(_)) => {
            cmd_ring.retire();
            result
        }
        Err(_) => result,
    }
}
