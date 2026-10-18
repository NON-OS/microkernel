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

//! Waiting for an isoch batch's last TD. Events of the same endpoint that
//! are not it are dropped rather than parked: they are the underrun the
//! stream had when its ring ran dry, or a missed TD, and no waiter will ever
//! ask for them. Other endpoints' events are parked for their own waiters.

use nonos_libc::Deadline;

use crate::constants::TRB_TYPE_TRANSFER_EVENT;
use crate::controller::park::SPIN_BUDGET;
use crate::error::{XhciError, XhciResult};
use crate::regs::runtime::erdp_program;
use crate::rings::event::{EventRing, IssuedTransfer};
use crate::trb::Trb;

/// A batch is at most 32 service intervals, 32 ms at full speed; a second
/// is a stream that has stopped, not one that is late.
const ISOCH_TIMEOUT_MS: u64 = 1_000;

/// The event of `issued`, whose place (`expect`) is given back.
pub(super) fn wait_isoch(
    intr_base: u64,
    issued: IssuedTransfer,
    evt: &mut EventRing,
) -> XhciResult<Trb> {
    if let Some(event) = evt.take_parked(issued) {
        return Ok(event);
    }
    let deadline = Deadline::after_ms(ISOCH_TIMEOUT_MS);
    let mut spins = 0u32;
    loop {
        spins = spins.saturating_add(1);
        if spins > SPIN_BUDGET && deadline.expired() {
            return Err(XhciError::TransferCompletionTimeout);
        }
        if !evt.has_event() {
            if spins > SPIN_BUDGET && evt.controller_failed() {
                return Err(XhciError::ControllerDead);
            }
            evt.irq.park(spins);
            continue;
        }
        let event = evt.current_trb();
        evt.advance();
        erdp_program(intr_base, evt.current_dequeue_phys(), true, 0);
        if event.get_type() != TRB_TYPE_TRANSFER_EVENT {
            continue;
        }
        if issued.completed_by(&event) {
            evt.settle(issued);
            return Ok(event);
        }
        if event.slot_id() != issued.slot || event.endpoint_id() != issued.dci {
            evt.park(event);
        }
    }
}
