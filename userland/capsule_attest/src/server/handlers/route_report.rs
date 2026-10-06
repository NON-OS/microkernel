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

//! A transport posting its route report.
//!
//! Who sent it is read from the kernel, not from the message: the IPC layer
//! gives the sender's pid, and the process table (which this capsule reads in
//! full under AttestRead) gives that pid's spawn name and capability mask. The
//! board then accepts the report only from net.nym about Nym or net.anon about
//! Anyone, each holding Network.

use nonos_route_proof::{Board, PostError};

use crate::protocol::{Request, E_INVAL, E_PERM};
use crate::server::respond;
use crate::state::live::identity;

pub fn run(
    out: &mut [u8],
    req: &Request,
    payload: &[u8],
    sender_pid: u32,
    board: &mut Board,
) -> usize {
    let Some((name, name_len, caps)) = identity(sender_pid) else {
        return respond::status(out, req, E_PERM);
    };
    let now = nonos_libc::mk_time_millis().max(0) as u64;
    match board.post(&name[..name_len], caps, payload, now) {
        Ok(_) => respond::status(out, req, 0),
        Err(PostError::Malformed) => respond::status(out, req, E_INVAL),
        Err(PostError::NotPermitted) => respond::status(out, req, E_PERM),
    }
}
