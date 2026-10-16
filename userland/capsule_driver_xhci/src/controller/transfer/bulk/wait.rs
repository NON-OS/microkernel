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
//! Waiting for one bulk TRB's transfer event. Events for other TRBs that
//! arrive first are parked for their own waiters; none is ever parked for
//! a bulk TRB, which only its own wait reads.
use nonos_libc::Deadline;

use crate::constants::TRB_TYPE_TRANSFER_EVENT;
use crate::controller::park::SPIN_BUDGET;
use crate::error::{XhciError, XhciResult};
use crate::regs::runtime::erdp_program;
use crate::rings::event::{EventRing, IssuedTransfer};
use crate::trb::Trb;
/// A USB stick answers a 4 KiB transfer in milliseconds; five seconds
/// covers a device that is still spinning up or flushing its cache.
const BULK_TIMEOUT_MS: u64 = 5_000;
pub(super) fn wait_bulk(
    intr_base: u64,
    issued: IssuedTransfer,
    evt_ring: &mut EventRing,
) -> XhciResult<Trb> {
    let deadline = Deadline::after_ms(BULK_TIMEOUT_MS);
    let mut spins = 0u32;
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
            return Ok(event);
        }
        evt_ring.park(event);
    }
}
