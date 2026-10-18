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

use super::transfer_event::{complete, short_on, stopped_early};
use crate::constants::TRB_TYPE_TRANSFER_EVENT;
use crate::controller::park::SPIN_BUDGET;
use crate::error::{XhciError, XhciResult};
use crate::regs::runtime::erdp_program;
use crate::rings::event::{EventRing, IssuedTransfer};

/// A control transfer gets 5 s (as observed, unverified): a slow stick or
/// card reader can miss SET_CONFIGURATION or CLEAR_FEATURE within a second.
const TRANSFER_TIMEOUT_MS: u64 = 5_000;

/// Wait for the event of `issued`, the last TRB of a transfer. `earlier`
/// are the transfer's other TRBs (a control transfer's setup and data
/// stages): a STALL or other error stops the transfer on one of them, the
/// controller reports it there and halts the endpoint, and the last TRB is
/// never reached. Such an event ends the wait at once with its code, where
/// it used to be parked while the wait ran to its timeout. A Short Packet
/// on one of them (an IN data stage with ISP set, xHCI 1.2 section
/// 4.10.1.1) is the transfer's residual, returned once the last TRB
/// completes; 0 when none came.
pub fn wait_transfer_completion(
    intr_base: u64,
    issued: IssuedTransfer,
    earlier: &[u64],
    evt_ring: &mut EventRing,
) -> XhciResult<u32> {
    let deadline = Deadline::after_ms(TRANSFER_TIMEOUT_MS);
    let (mut spins, mut residual) = (0u32, 0u32);
    loop {
        spins = spins.saturating_add(1);
        if spins > SPIN_BUDGET && deadline.expired() {
            return Err(XhciError::TransferCompletionTimeout);
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
        if event.get_type() != TRB_TYPE_TRANSFER_EVENT {
            continue;
        }
        if issued.completed_by(&event) {
            return complete(event.completion_code()).map(|()| residual);
        }
        if stopped_early(&event, issued, earlier) {
            return Err(XhciError::TransferCompletionFailed(event.completion_code()));
        }
        match short_on(&event, issued, earlier) {
            Some(left) => residual = left,
            None => evt_ring.park(event),
        }
    }
}
