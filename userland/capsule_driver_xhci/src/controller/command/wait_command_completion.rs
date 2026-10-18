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
use crate::constants::CC_COMMAND_RING_STOPPED;
use crate::error::XhciResult;
use crate::rings::event::EventRing;

// Linux XHCI_CMD_DEFAULT_TIMEOUT: an Address Device carries a SET_ADDRESS to
// the device, which a slow device can take well past a second to answer.
pub(super) const COMPLETION_TIMEOUT_MS: u64 = 5_000;

/// Wait for the completion of the command TRB at `issued_phys`: the event
/// whose pointer names it. Command Ring Stopped is never that event, though
/// its pointer can name it: it reports the ring stopped, and its pointer is
/// where the ring will go on, the TRB the next command is written to.
pub fn wait_command_completion(
    intr_base: u64,
    issued_phys: u64,
    evt_ring: &mut EventRing,
) -> XhciResult<CommandCompletion> {
    wait_command_within(intr_base, issued_phys, evt_ring, COMPLETION_TIMEOUT_MS)
}

/// The same, given up after `timeout_ms`.
pub(super) fn wait_command_within(
    intr_base: u64,
    issued_phys: u64,
    evt_ring: &mut EventRing,
    timeout_ms: u64,
) -> XhciResult<CommandCompletion> {
    let deadline = Deadline::after_ms(timeout_ms);
    loop {
        let event = next_completion(intr_base, evt_ring, &deadline)?;
        if event.completion_code() == CC_COMMAND_RING_STOPPED || !names(&event, issued_phys) {
            continue;
        }
        return answer(&event);
    }
}
