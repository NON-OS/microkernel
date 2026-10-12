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
//! One request answered. OPEN takes the reader for its caller (a holder
//! the kernel says is gone does not keep it); every other op but the
//! health check is the holder's alone. A command the reader failed is
//! answered -5 with `bError u8, icc u8`, so the holder can tell a card
//! pulled (icc 2) from one that refused.

use crate::card::{ask, Fault};
use crate::ccid::{GET_SLOT_STATUS, ICC_POWER_OFF, ICC_POWER_ON, XFR_BLOCK};
use crate::device::Dev;
use crate::frame::{reply, Request};
use crate::session::{close, open};
use crate::wire::*;

pub fn answer(d: &mut Dev, peer: u32, req: &Request, body: &[u8], tx: &mut [u8]) -> usize {
    let fault = |tx: &mut [u8], (e, why): Fault| reply(tx, req, e, why.as_ref().map_or(&[], |w| w));
    if req.op == OP_HEALTHCHECK {
        return reply(tx, req, if body.is_empty() { 0 } else { E_INVAL }, &[]);
    }
    let Some(p) = d.port() else { return reply(tx, req, E_NODEV, &[]) };
    let (ids, max_slot, features) = (p.ids, p.r.max_slot, p.r.features.to_le_bytes());
    if req.op == OP_OPEN {
        let (v, p, f) = (ids.0.to_le_bytes(), ids.1.to_le_bytes(), features);
        let opened = [v[0], v[1], p[0], p[1], max_slot, 0, 0, 0, f[0], f[1], f[2], f[3]];
        return match open(d, peer, body) {
            Ok(()) => reply(tx, req, 0, &opened),
            Err(e) => reply(tx, req, e, &[]),
        };
    }
    if let Err(e) = d.owner.own(peer) {
        return reply(tx, req, e, &[]);
    }
    let slot = body.first().copied().unwrap_or(0);
    let asked = match (req.op, body.len()) {
        (OP_CLOSE, 0) => {
            close(d);
            return reply(tx, req, 0, &[]);
        }
        (OP_POWER_ON, 1) => ask(d, ICC_POWER_ON, slot, &[]).inspect(|_| d.slots.power(slot, true)),
        (OP_POWER_OFF, 1) => {
            ask(d, ICC_POWER_OFF, slot, &[]).inspect(|_| d.slots.power(slot, false))
        }
        (OP_STATUS, 1) => ask(d, GET_SLOT_STATUS, slot, &[]).map(|(_, icc)| {
            let changed = d.slots.take_changed(slot) as u8;
            (alloc::vec![icc, changed, 0, 0], icc)
        }),
        (OP_TRANSMIT, 5..=CHUNK) if body[1..4] == [0; 3] => ask(d, XFR_BLOCK, slot, &body[4..]),
        _ => Err((E_INVAL, None)),
    };
    match asked {
        Ok((data, _)) if data.len() > CHUNK => reply(tx, req, E_MSGSIZE, &[]),
        Ok((data, _)) => reply(tx, req, 0, &data),
        Err(f) => fault(tx, f),
    }
}
