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

//! `OP_ISOCH_OUT`: queue a batch on the slot's isoch endpoint. Request
//! `slot, 0, packets, 0`, a u16 length per packet, then their bytes. Reply
//! `cc, 0, 0, 0`: the completion of the batch sent before this one, success
//! (1) or missed service (23), which the class driver may count as a
//! dropout. A batch that failed or never finished recovers the endpoint and
//! answers E_IO; the next one starts the stream again.

use crate::controller::isoch_out;
use crate::error::XhciError;
use crate::protocol::{Request, E_INVAL, E_IO, ISOCH_HEADER_LEN, ISOCH_PACKETS_MAX};
use crate::server::context::Context;
use crate::server::error::reply_with_status;
use crate::server::handlers::bulk::reply::send_prefix;
use crate::server::handlers::recover::recover_after;

pub fn out(ctx: &mut Context, req: &Request, body: &[u8], tx: &mut [u8]) {
    let Some(&[slot, _, packets, _]) = body.first_chunk::<ISOCH_HEADER_LEN>() else {
        return reply_with_status(tx, req, E_INVAL);
    };
    let n = packets as usize;
    let data_at = ISOCH_HEADER_LEN + 2 * n;
    if n == 0 || n > ISOCH_PACKETS_MAX || body.len() < data_at {
        return reply_with_status(tx, req, E_INVAL);
    }
    let mut lens = [0u16; ISOCH_PACKETS_MAX];
    for (i, l) in lens[..n].iter_mut().enumerate() {
        let at = ISOCH_HEADER_LEN + 2 * i;
        *l = u16::from_le_bytes([body[at], body[at + 1]]);
    }
    let d = &mut ctx.driver;
    let (db, intr, max_slots) =
        (d.layout.doorbell_base, d.layout.primary_intr_base, d.layout.max_slots);
    let Some(pipe) = d.slots.resources_mut(slot, max_slots).and_then(|r| r.isoch.as_mut()) else {
        return reply_with_status(tx, req, E_INVAL);
    };
    let dci = pipe.ep.dci;
    match isoch_out(db, intr, &mut d.event_ring, pipe, slot, &lens[..n], &body[data_at..]) {
        Ok(cc) => send_prefix(tx, req, [cc, 0, 0, 0]),
        Err(XhciError::ControllerUnsupported) => reply_with_status(tx, req, E_INVAL),
        Err(e) => {
            // The batches point into the pipe's own memory, none to let go.
            recover_after(ctx, slot, dci, e, None);
            reply_with_status(tx, req, E_IO)
        }
    }
}
