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

//! Routing a request to the handler that answers it.

use crate::manager::Manager;
use crate::protocol::{
    E_BAD_OP, E_OK, OP_BUILD_CIRCUIT, OP_CIRCUIT_PATH, OP_CLOSE_CIRCUIT, OP_CLOSE_STREAM,
    OP_HEALTHCHECK, OP_OPEN_STREAM, OP_RECV, OP_SEND, OP_STATUS, OP_SYNC_DIRECTORY,
};

use super::handlers::{circuit_path, close_circuit, close_stream, open, recv, send, status};
use super::parse_req::Request;

/// Answer one request, returning the errno and body length to reply with.
pub fn dispatch(
    state: &mut Manager,
    req: &Request,
    body: &[u8],
    now: u64,
    tx: &mut [u8],
) -> (u16, u32) {
    match req.op {
        OP_HEALTHCHECK => (E_OK, 0),
        OP_STATUS => (E_OK, status(state, tx)),
        OP_OPEN_STREAM => open(state, body, now, tx),
        OP_SEND => send(state, body, tx),
        OP_RECV => recv(state, body, tx),
        OP_CLOSE_STREAM => (close_stream(state, body), 0),
        OP_CLOSE_CIRCUIT => (close_circuit(state, body), 0),
        /*
         * Both of these are answered by the idle path, which fetches and builds
         * whenever there is nothing to serve. A caller asking for them is telling
         * the capsule it is wanted now, and the honest answer is that it is
         * already in hand rather than a second fetch on top of the first.
         */
        OP_SYNC_DIRECTORY | OP_BUILD_CIRCUIT => (E_OK, 0),
        OP_CIRCUIT_PATH => circuit_path(state, body, tx),
        _ => (E_BAD_OP, 0),
    }
}
