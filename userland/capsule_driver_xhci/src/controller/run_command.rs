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
//! aborted so the commands after it are not stuck behind it on the ring.
use super::ring_doorbell::ring_doorbell;
use super::wait_command_completion::{wait_command_completion, CommandCompletion};
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
    if result.is_err_and(|e| e == XhciError::CommandCompletionTimeout) {
        let _ = cmd_ring.abort();
    }
    result
}
