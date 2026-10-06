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
mod reply;
use crate::controller::{poll_interrupt_in, IntrPoll};
use crate::protocol::{Request, E_INVAL, E_IO, HID_REPORT_MAX, INTERRUPT_IN_REQUEST_LEN};
use crate::server::context::Context;
use crate::server::error::reply_with_status;
use crate::server::handlers::recover::recover_after;
pub fn handle(ctx: &mut Context, req: &Request, body: &[u8], tx: &mut [u8]) {
    if body.len() != INTERRUPT_IN_REQUEST_LEN {
        reply_with_status(tx, req, E_INVAL);
        return;
    }
    let (slot, dci) = (body[0], body[1]);
    let length = u16::from_le_bytes([body[2], body[3]]);
    if length as usize > HID_REPORT_MAX {
        reply_with_status(tx, req, E_INVAL);
        return;
    }
    match poll(ctx, slot, dci, length) {
        Ok(IntrPoll::Complete(n)) => reply::reply_report(ctx, req, slot, dci, tx, n),
        Ok(IntrPoll::Pending) => reply::reply_pending(tx, req),
        Err(e) => {
            // A report that ended in an error halted the endpoint; it is
            // brought back here so the next poll arms it again.
            recover_after(ctx, slot, dci, e);
            reply_with_status(tx, req, E_IO)
        }
    }
}
fn poll(ctx: &mut Context, slot: u8, dci: u8, length: u16) -> crate::error::XhciResult<IntrPoll> {
    let doorbell_base = ctx.driver.layout.doorbell_base;
    let intr_base = ctx.driver.layout.primary_intr_base;
    let max_slots = ctx.driver.layout.max_slots;
    let res = ctx
        .driver
        .slots
        .resources_mut(slot, max_slots)
        .ok_or(crate::error::XhciError::ControllerUnsupported)?;
    poll_interrupt_in(doorbell_base, intr_base, &mut ctx.driver.event_ring, res, dci, length)
}
