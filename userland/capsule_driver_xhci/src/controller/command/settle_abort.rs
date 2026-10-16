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
use nonos_libc::Deadline;

use super::next_completion::{answer, names, next_completion, CommandCompletion};
use super::wait_command_completion::COMPLETION_TIMEOUT_MS;
use crate::constants::{CC_COMMAND_ABORTED, CC_COMMAND_RING_STOPPED};
use crate::error::{XhciError, XhciResult};
use crate::rings::event::EventRing;

/// Consume what giving up the command at `issued` left on the event ring,
/// matched by pointer and code: its Command Aborted, its own completion if
/// it finished before the abort took, and, when `stopped`, the Command Ring
/// Stopped that ends them (xHCI 1.2 section 4.6.1.2), waited for as Linux
/// waits, at most `COMPLETION_TIMEOUT_MS`. A ring that was not running
/// posts no Ring Stopped, so only what is already there is read. None of
/// these is left to answer a later command or to block the ring's head.
///
/// The answer is the command's own completion when it came, so a command
/// the controller carried out is reported as carried out; otherwise the
/// timeout.
pub(super) fn settle_abort(
    intr_base: u64,
    evt_ring: &mut EventRing,
    issued: u64,
    stopped: bool,
) -> XhciResult<CommandCompletion> {
    let wait_ms = if stopped { COMPLETION_TIMEOUT_MS } else { 0 };
    let deadline = Deadline::after_ms(wait_ms);
    let mut got = Err(XhciError::CommandCompletionTimeout);
    while let Ok(event) = next_completion(intr_base, evt_ring, &deadline) {
        match event.completion_code() {
            CC_COMMAND_RING_STOPPED => break,
            CC_COMMAND_ABORTED => {}
            _ if names(&event, issued) => got = answer(&event),
            _ => {}
        }
    }
    got
}
