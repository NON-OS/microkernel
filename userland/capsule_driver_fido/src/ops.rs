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
//! One request answered. OPEN takes the key for its caller (a holder the
//! kernel says is gone does not keep it); every other op but the health
//! check is the holder's alone. Requests and answers pass through unread:
//! this driver never answers for the key, and never for its user.

use crate::ctaphid::Fault;
use crate::device::Dev;
use crate::frame::{reply, Request};
use crate::request::{cancel, start};
use crate::session::{close, open};
use crate::wire::*;

pub fn answer(d: &mut Dev, peer: u32, req: &Request, body: &[u8], tx: &mut [u8]) -> usize {
    let status = |tx: &mut [u8], s: Result<(), i32>| reply(tx, req, s.err().unwrap_or(0), &[]);
    if req.op == OP_HEALTHCHECK {
        return status(tx, if body.is_empty() { Ok(()) } else { Err(E_INVAL) });
    }
    let Some(p) = d.port() else { return reply(tx, req, E_NODEV, &[]) };
    let ((v, k), c) = ((p.ids.0.to_le_bytes(), p.ids.1.to_le_bytes()), p.ch);
    if req.op == OP_OPEN {
        let opened = [v[0], v[1], k[0], k[1], c.caps, c.version[0], c.version[1], c.version[2]];
        return status_or(tx, req, open(d, peer, body), &opened);
    }
    if let Err(e) = d.owner.own(peer) {
        return reply(tx, req, e, &[]);
    }
    match (req.op, body.len()) {
        (OP_CLOSE, 0) => {
            close(d);
            status(tx, Ok(()))
        }
        (OP_CBOR, 1..=CHUNK) => status(tx, start(d, body)),
        (OP_CANCEL, 0) => status(tx, cancel(d)),
        (OP_RESULT, 0) => match d.job.result() {
            Ok(a) if a.len() > CHUNK => reply(tx, req, E_MSGSIZE, &[]),
            Ok(a) => reply(tx, req, 0, &a),
            Err((e, why)) => fault(tx, req, (e, why)),
        },
        _ => reply(tx, req, E_INVAL, &[]),
    }
}

fn status_or(tx: &mut [u8], req: &Request, s: Result<(), i32>, ok: &[u8]) -> usize {
    match s {
        Ok(()) => reply(tx, req, 0, ok),
        Err(e) => reply(tx, req, e, &[]),
    }
}

/// -11 with the keep-alive status, -5 with the key's CTAPHID error code.
fn fault(tx: &mut [u8], req: &Request, (e, why): Fault) -> usize {
    reply(tx, req, e, why.as_slice())
}
