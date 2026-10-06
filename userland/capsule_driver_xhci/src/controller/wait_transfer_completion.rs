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

use crate::constants::{CC_SHORT_PACKET, CC_SUCCESS, TRB_TYPE_TRANSFER_EVENT};
use crate::controller::park::{park_step, SPIN_BUDGET};
use crate::error::{XhciError, XhciResult};
use crate::regs::runtime::erdp_program;
use crate::rings::event::{EventRing, IssuedTransfer};
use crate::trb::Trb;

/// A control transfer gets 5 s, as Linux USB_CTRL_SET_TIMEOUT: a slow stick or
/// card reader can miss SET_CONFIGURATION or CLEAR_FEATURE within a second.
const TRANSFER_TIMEOUT_MS: u64 = 5_000;

/// Wait for the event of `issued`, the last TRB of a transfer. `earlier`
/// are the transfer's other TRBs (a control transfer's setup and data
/// stages): a STALL or other error stops the transfer on one of them, the
/// controller reports it there and halts the endpoint, and the last TRB is
/// never reached. Such an event ends the wait at once with its code, where
/// it used to be parked while the wait ran to its timeout.
pub fn wait_transfer_completion(
    intr_base: u64,
    issued: IssuedTransfer,
    earlier: &[u64],
    evt_ring: &mut EventRing,
) -> XhciResult<()> {
    let deadline = Deadline::after_ms(TRANSFER_TIMEOUT_MS);
    let mut spins = 0u32;
    loop {
        spins = spins.saturating_add(1);
        if spins > SPIN_BUDGET && deadline.expired() {
            return Err(XhciError::TransferCompletionTimeout);
        }
        if !evt_ring.has_event() {
            park_step(spins);
            continue;
        }
        let event = evt_ring.current_trb();
        evt_ring.advance();
        erdp_program(intr_base, evt_ring.current_dequeue_phys(), true, 0);
        if event.get_type() != TRB_TYPE_TRANSFER_EVENT {
            continue;
        }
        if issued.completed_by(&event) {
            return complete(event.completion_code());
        }
        if stopped_early(&event, issued, earlier) {
            return Err(XhciError::TransferCompletionFailed(event.completion_code()));
        }
        evt_ring.park(event);
    }
}
fn complete(code: u8) -> XhciResult<()> {
    match code {
        CC_SUCCESS | CC_SHORT_PACKET => Ok(()),
        other => Err(XhciError::TransferCompletionFailed(other)),
    }
}
/// Whether `event` is an error on one of `earlier`, on the same slot and
/// endpoint as `issued`.
pub fn stopped_early(event: &Trb, issued: IssuedTransfer, earlier: &[u64]) -> bool {
    let cc = event.completion_code();
    let at = event.get_pointer() & !0xF;
    event.slot_id() == issued.slot
        && event.endpoint_id() == issued.dci
        && cc != CC_SUCCESS
        && cc != CC_SHORT_PACKET
        && earlier.iter().any(|&p| p & !0xF == at)
}
