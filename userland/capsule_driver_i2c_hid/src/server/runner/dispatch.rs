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

use crate::protocol::{E_BAD_OP, E_INVAL, OP_DESCRIPTOR, OP_HEALTHCHECK};
use crate::server::{handlers, respond};
use crate::state::State;

pub(super) fn dispatch(
    state: &mut State,
    sender_pid: u32,
    req: crate::protocol::Request,
    body: &[u8],
    tx: &mut [u8],
) {
    match req.op {
        OP_HEALTHCHECK if body.is_empty() => handlers::health::handle(state, sender_pid, &req, tx),
        OP_DESCRIPTOR if body.is_empty() => {
            handlers::descriptor::handle(state, sender_pid, &req, tx)
        }
        _ if body.is_empty() => {
            let _ = respond::send(sender_pid, &req, E_BAD_OP, &[], tx);
        }
        _ => {
            let _ = respond::send(sender_pid, &req, E_INVAL, &[], tx);
        }
    }
}
