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
//! One request answered. OPEN takes the host for its caller (a holder the
//! kernel says is gone does not keep it); TRANSFER and CLOSE are the
//! holder's alone. Bytes pass through unread.

use nonos_libc::mk_pid_alive;

use crate::facts::INPUT_CLOCK_HZ;
use crate::frame::{reply, Request};
use crate::owner::Owner;
use crate::setup::Host;
use crate::transfer::transfer;
use crate::wire::*;

pub fn answer(
    h: &Host,
    o: &mut Owner,
    peer: u32,
    req: &Request,
    body: &[u8],
    tx: &mut [u8],
) -> usize {
    let status = |tx: &mut [u8], s: Result<(), i32>| reply(tx, req, s.err().unwrap_or(0), &[]);
    match (req.op, body.len()) {
        (OP_HEALTHCHECK, 0) => status(tx, Ok(())),
        (OP_OPEN, 0) => match o.open(peer, |p| mk_pid_alive(p)) {
            Ok(()) => {
                let (d, c) = (h.device.to_le_bytes(), INPUT_CLOCK_HZ.to_le_bytes());
                reply(tx, req, 0, &[d[0], d[1], 0, 0, c[0], c[1], c[2], c[3]])
            }
            Err(e) => reply(tx, req, e, &[]),
        },
        _ if o.own(peer).is_err() => reply(tx, req, -1, &[]),
        (OP_CLOSE, 0) => {
            o.close();
            status(tx, Ok(()))
        }
        (OP_TRANSFER, 9..=CHUNK) => {
            let hz = u32::from_le_bytes([body[4], body[5], body[6], body[7]]);
            match transfer(h, body[0], hz, &body[8..]) {
                Ok(rx) => reply(tx, req, 0, &rx),
                Err(e) => reply(tx, req, e, &[]),
            }
        }
        _ => reply(tx, req, E_INVAL, &[]),
    }
}
