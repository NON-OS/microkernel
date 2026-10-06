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
use super::IntrPoll;
use crate::constants::{CC_SHORT_PACKET, CC_SUCCESS, TRB_TYPE_TRANSFER_EVENT};
use crate::error::{XhciError, XhciResult};
use crate::regs::runtime::erdp_program;
use crate::rings::event::{EventRing, IssuedTransfer};
use crate::slots::InterruptEndpoint;
use crate::trb::Trb;
pub fn scan(
    intr_base: u64,
    evt_ring: &mut EventRing,
    slot: u8,
    ep: &mut InterruptEndpoint,
    length: u16,
) -> XhciResult<IntrPoll> {
    let issued = IssuedTransfer { phys: ep.armed.unwrap_or(0), slot, dci: ep.dci };
    if let Some(event) = evt_ring.take_parked(issued) {
        ep.armed = None;
        return completed(&event, length);
    }
    if !evt_ring.has_event() {
        return Ok(IntrPoll::Pending);
    }
    let event = evt_ring.current_trb();
    let armed = issued.completed_by(&event);
    if !armed && event.get_type() != TRB_TYPE_TRANSFER_EVENT {
        return Ok(IntrPoll::Pending);
    }
    evt_ring.advance();
    erdp_program(intr_base, evt_ring.current_dequeue_phys(), true, 0);
    if !armed {
        /*
         * Another endpoint's completion: kept for its own poll, so it
         * neither blocks this one nor is lost.
         */
        evt_ring.park(event);
        return Ok(IntrPoll::Pending);
    }
    ep.armed = None;
    completed(&event, length)
}
/// The report the event completes, or the error it ended in.
fn completed(event: &Trb, length: u16) -> XhciResult<IntrPoll> {
    match event.completion_code() {
        CC_SUCCESS | CC_SHORT_PACKET => Ok(IntrPoll::Complete(transferred(event, length))),
        cc => Err(XhciError::TransferCompletionFailed(cc)),
    }
}
/// The bytes the report holds: the request less the residual the event
/// reports, the residual clamped to the request. A residual past the request
/// says more bytes stayed behind than were asked for, so none of the buffer
/// is vouched for and none is handed up, as the bulk path reads it too.
fn transferred(event: &Trb, length: u16) -> u16 {
    let residual = event.d2 & 0x00FF_FFFF;
    let req = length as u32;
    (req - residual.min(req)) as u16
}
