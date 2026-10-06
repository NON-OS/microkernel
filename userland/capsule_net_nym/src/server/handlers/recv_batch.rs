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

//! A read that takes everything delivered that fits in one answer.

use super::recv::CLIENT_WAIT_MS;
use super::recv_drain::drain_stream;
use crate::protocol::batch::RECORD_HEADER;
use crate::protocol::{E_BAD_LEN, E_NO_SESSION, E_OK, E_RX_EMPTY, OP_RECV_BATCH};
use crate::server::handlers::io::u32_at;
use crate::server::parse_req::Request;
use crate::server::respond::respond;
use crate::state::TABLE;

/// Header of every answer this capsule sends.
const HDR: usize = 20;

/// Body: the session id, then optionally how long to wait on an empty link
/// in milliseconds. The caller names the wait because only it knows how long
/// its own caller is prepared to block; it is capped all the same, since the
/// server answers nobody else while it waits.
pub fn handle(pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    let session_id = match u32_at(body, 0) {
        Ok(id) => id,
        Err(e) => return respond(pid, OP_RECV_BATCH, e, req.request_id, 0, tx),
    };
    let wait = u32_at(body, 4).map_or(CLIENT_WAIT_MS, |ms| (ms as i64).min(CLIENT_WAIT_MS));
    if tx.len() <= HDR + RECORD_HEADER {
        return respond(pid, OP_RECV_BATCH, E_BAD_LEN, req.request_id, 0, tx);
    }
    let Some(mut written) = fill(pid, session_id, &mut tx[HDR..]) else {
        return respond(pid, OP_RECV_BATCH, E_NO_SESSION, req.request_id, 0, tx);
    };
    // Whatever the gateway already holds is taken now rather than on the
    // next read, and the link is only waited on when there was nothing to
    // give: a reader with bytes in hand should not be held for more.
    drain_stream(if written == 0 { wait } else { 0 });
    written += fill(pid, session_id, &mut tx[HDR + written..]).unwrap_or(0);
    if written == 0 {
        return respond(pid, OP_RECV_BATCH, E_RX_EMPTY, req.request_id, 0, tx);
    }
    respond(pid, OP_RECV_BATCH, E_OK, req.request_id, written as u32, tx);
}

fn fill(pid: u32, id: u32, out: &mut [u8]) -> Option<usize> {
    TABLE.lock().with_mut(pid, id, |s| s.fill_records(out))
}
