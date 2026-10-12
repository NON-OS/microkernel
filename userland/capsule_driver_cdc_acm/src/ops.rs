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
//! One request answered. OPEN takes the line for its caller (a holder
//! the kernel says is gone does not keep it); every other op but the
//! health check is the holder's alone.

use nonos_usbnet::Bus;

use crate::control::{brk, line, lines};
use crate::device::Dev;
use crate::frame::{reply, Request};
use crate::session::{close, open};
use crate::wire::*;

pub fn answer(d: &mut Dev, peer: u32, req: &Request, body: &[u8], tx: &mut [u8]) -> usize {
    let status = |tx: &mut [u8], s: Result<(), i32>| reply(tx, req, s.err().unwrap_or(0), &[]);
    if req.op == OP_HEALTHCHECK {
        return status(tx, if body.is_empty() { Ok(()) } else { Err(E_INVAL) });
    }
    let Some(p) = d.port() else { return reply(tx, req, E_NODEV, &[]) };
    let (ids, protocol, caps) = (p.ids, p.f.protocol, p.f.capabilities);
    if req.op == OP_OPEN {
        return match open(d, peer, body) {
            Ok(()) => {
                let (v, p) = (ids.0.to_le_bytes(), ids.1.to_le_bytes());
                reply(tx, req, 0, &[v[0], v[1], p[0], p[1], protocol, caps, 0, 0])
            }
            Err(e) => reply(tx, req, e, &[]),
        };
    }
    if let Err(e) = d.owner.own(peer) {
        return reply(tx, req, e, &[]);
    }
    let two = |b: &[u8]| u16::from_le_bytes([b[0], b[1]]);
    match (req.op, body.len()) {
        (OP_CLOSE, 0) => {
            close(d);
            status(tx, Ok(()))
        }
        (OP_WRITE, 1..=CHUNK) => match d.bus().map(|mut bus| bus.bulk_out(body)) {
            Some(Ok(n)) => reply(tx, req, 0, &(n as u32).to_le_bytes()),
            _ => reply(tx, req, E_IO, &[]),
        },
        (OP_READ, 2) => reply(tx, req, 0, &d.rx.take((two(body) as usize).min(CHUNK))),
        (OP_SET_LINE, 7) => status(tx, line(d, body)),
        (OP_SET_CONTROL, 2) => status(tx, lines(d, two(body))),
        (OP_BREAK, 2) => status(tx, brk(d, two(body))),
        (OP_STATUS, 0) => {
            let (s, q) = (d.state.take().to_le_bytes(), (d.rx.queued() as u32).to_le_bytes());
            reply(tx, req, 0, &[s[0], s[1], 0, 0, q[0], q[1], q[2], q[3]])
        }
        _ => reply(tx, req, E_INVAL, &[]),
    }
}
