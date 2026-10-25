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
//! `OP_INTERRUPT_OUT` (`slot, dci, len_le16, data`): one report sent to an
//! interrupt OUT endpoint whose ring OP_ALLOC_TRANSFER_RING made. Replies
//! `moved_le16, 0, 0`. A report that failed or never finished has its
//! endpoint brought back before the reply (E_IO), so the next is served.

use crate::controller::interrupt_out;
use crate::protocol::{Request, E_INVAL, E_IO};
use crate::server::context::Context;
use crate::server::error::reply_with_status;
use crate::server::handlers::bulk::reply::send_prefix;
use crate::server::handlers::recover::recover_after;

const HEADER: usize = 4;

pub fn handle(ctx: &mut Context, req: &Request, body: &[u8], tx: &mut [u8]) {
    let Some(data) = body.get(HEADER..) else { return reply_with_status(tx, req, E_INVAL) };
    let (slot, dci, len) = (body[0], body[1], u16::from_le_bytes([body[2], body[3]]));
    if data.is_empty() || data.len() != len as usize {
        return reply_with_status(tx, req, E_INVAL);
    }
    let d = &mut ctx.driver;
    let Some(res) = d.slots.resources_mut(slot, d.layout.max_slots) else {
        return reply_with_status(tx, req, E_INVAL);
    };
    let (doorbell, intr) = (d.layout.doorbell_base, d.layout.primary_intr_base);
    match interrupt_out(doorbell, intr, &mut d.event_ring, res, dci, data) {
        Ok(n) => send_prefix(tx, req, [n as u8, (n >> 8) as u8, 0, 0]),
        Err(crate::error::XhciError::ControllerUnsupported) => reply_with_status(tx, req, E_INVAL),
        Err(e) => {
            recover_after(ctx, slot, dci, e, None);
            reply_with_status(tx, req, E_IO)
        }
    }
}
