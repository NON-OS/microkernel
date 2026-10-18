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

use crate::constants::{CC_SUCCESS, TRB_TYPE_CMD_COMPLETION_EVENT, TRB_TYPE_TRANSFER_EVENT};
use crate::controller::park::SPIN_BUDGET;
use crate::error::{XhciError, XhciResult};
use crate::regs::runtime::erdp_program;
use crate::rings::event::EventRing;
use crate::trb::Trb;

#[derive(Clone, Copy)]
pub struct CommandCompletion {
    pub slot_id: u8,
}

/// The next Command Completion Event, handed back to the controller as it
/// is read: the timeout once `deadline` has passed, `ControllerDead` at
/// once when the controller failed. A transfer event met on the way is
/// parked for its own waiter; any other event is passed over.
pub(super) fn next_completion(
    intr_base: u64,
    evt_ring: &mut EventRing,
    deadline: &Deadline,
) -> XhciResult<Trb> {
    let mut spins = 0u32;
    loop {
        spins = spins.saturating_add(1);
        if spins > SPIN_BUDGET && deadline.expired() {
            return Err(XhciError::CommandCompletionTimeout);
        }
        if !evt_ring.has_event() {
            if spins > SPIN_BUDGET && evt_ring.controller_failed() {
                return Err(XhciError::ControllerDead);
            }
            evt_ring.irq.park(spins);
            continue;
        }
        let event = evt_ring.current_trb();
        evt_ring.advance();
        erdp_program(intr_base, evt_ring.current_dequeue_phys(), true, 0);
        match event.get_type() {
            TRB_TYPE_CMD_COMPLETION_EVENT => return Ok(event),
            TRB_TYPE_TRANSFER_EVENT => evt_ring.park(event),
            _ => {}
        }
    }
}

/// Whether `event` names the command TRB at `issued`. Pointer bits 3:0
/// are reserved, so they are not compared.
pub(super) fn names(event: &Trb, issued: u64) -> bool {
    event.get_pointer() & !0xF == issued & !0xF
}

/// The command's answer, from the event that names it.
pub(super) fn answer(event: &Trb) -> XhciResult<CommandCompletion> {
    match event.completion_code() {
        CC_SUCCESS => Ok(CommandCompletion { slot_id: event.slot_id() }),
        cc => Err(XhciError::CommandCompletionFailed(cc)),
    }
}
