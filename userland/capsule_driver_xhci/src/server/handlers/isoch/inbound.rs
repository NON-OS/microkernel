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

//! `OP_ISOCH_IN`: queue a batch on the slot's isoch IN endpoint and answer
//! with the batch queued before it. Request `slot, 0, packets, 0`: that many
//! service intervals, each TD one interval's payload. Reply `cc, packets, 0,
//! 0`, then ISOCH_PACKETS_MAX u16 lengths (the first `packets` used), then
//! the packets' bytes one after another. `cc` is success (1), or missed
//! service (23) when a packet brought nothing, which the class driver may
//! count as a dropout. The first batch answers no packets. A batch that
//! failed or never finished recovers the endpoint and answers E_IO.

use crate::controller::isoch_in;
use crate::error::XhciError;
use crate::protocol::{Request, E_INVAL, E_IO, ISOCH_HEADER_LEN, ISOCH_IN_MAX, ISOCH_PACKETS_MAX};
use crate::server::context::Context;
use crate::server::error::reply_with_status;
use crate::server::handlers::bulk::reply::{send_with_data, DATA_AT};
use crate::server::handlers::recover::recover_after;

const LENS_LEN: usize = 2 * ISOCH_PACKETS_MAX;

pub fn inbound(ctx: &mut Context, req: &Request, body: &[u8], tx: &mut [u8]) {
    let Ok(&[slot, _, packets, _]) = <&[u8; ISOCH_HEADER_LEN]>::try_from(body) else {
        return reply_with_status(tx, req, E_INVAL);
    };
    let d = &mut ctx.driver;
    let bases = (d.layout.doorbell_base, d.layout.primary_intr_base);
    let max_slots = d.layout.max_slots;
    let Some(pipe) = d.slots.resources_mut(slot, max_slots).and_then(|r| r.isoch_in.as_mut())
    else {
        return reply_with_status(tx, req, E_INVAL);
    };
    let dci = pipe.ep.dci;
    let mut lens = [0u16; ISOCH_PACKETS_MAX];
    let out = &mut tx[DATA_AT + LENS_LEN..DATA_AT + LENS_LEN + ISOCH_IN_MAX];
    let (slot_n, ev) = ((slot, packets as usize), &mut d.event_ring);
    match isoch_in(bases, ev, pipe, slot_n, &mut lens, out) {
        Ok((cc, n, bytes)) => {
            for (i, l) in lens.iter().enumerate() {
                tx[DATA_AT + 2 * i..DATA_AT + 2 * i + 2].copy_from_slice(&l.to_le_bytes());
            }
            send_with_data(tx, req, [cc, n as u8, 0, 0], LENS_LEN + bytes);
        }
        Err(XhciError::ControllerUnsupported) => reply_with_status(tx, req, E_INVAL),
        Err(e) => {
            recover_after(ctx, slot, dci, e);
            reply_with_status(tx, req, E_IO)
        }
    }
}
