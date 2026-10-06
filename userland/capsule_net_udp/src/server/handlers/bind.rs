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

use crate::protocol::{E_BAD_LEN, E_OK, E_PORT_IN_USE, OP_BIND};
use crate::server::parse_req::Request;
use crate::server::respond::respond;
use crate::state::{BindEntry, TableError, STATE};

fn alive(pid: u32) -> bool {
    nonos_libc::mk_pid_alive(pid)
}

// Body: 2-byte local port (LE).
pub fn handle(sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    if body.len() < 2 {
        let _ = respond(sender_pid, OP_BIND, E_BAD_LEN, req.request_id, 0, tx);
        return;
    }
    let port = u16::from_le_bytes([body[0], body[1]]);
    let mut binds = STATE.binds.lock();
    let mut placed = binds.insert(BindEntry::new(sender_pid, port));
    // The port, or the last free place, may be held by a client that ended.
    if placed.is_err() && binds.take_dead(alive) > 0 {
        placed = binds.insert(BindEntry::new(sender_pid, port));
    }
    drop(binds);
    let errno = match placed {
        Ok(()) => E_OK,
        Err(TableError::InUse) | Err(TableError::Full) => E_PORT_IN_USE,
        Err(TableError::NotFound) => E_PORT_IN_USE,
    };
    let _ = respond(sender_pid, OP_BIND, errno, req.request_id, 0, tx);
}
