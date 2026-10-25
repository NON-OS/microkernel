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
//! One request: parsed, dispatched, answered, and its bytes wiped.

use crate::protocol::E_BAD_OP;

use super::handlers;
use super::parse_req::{parse, refused};
use super::respond::respond;

/// Serve the message `sender` left in `rx`. A request may carry a secret (an
/// identity seed, a credential), so none stays in the buffer for the next
/// message to land beside, whatever the request came to.
pub(super) fn serve_one(sender: u32, rx: &mut [u8], tx: &mut [u8]) {
    match parse(rx) {
        Ok((req, body)) => {
            if !handlers::dispatch(sender, &req, body, tx) {
                respond(sender, req.op, E_BAD_OP, req.request_id, 0, tx);
            }
        }
        Err(errno) => {
            let req = refused(rx);
            respond(sender, req.op, errno, req.request_id, 0, tx);
        }
    }
    crate::crypto::wipe::wipe(rx);
}
