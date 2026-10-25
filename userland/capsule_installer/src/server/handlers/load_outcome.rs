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

use alloc::vec::Vec;

use super::load_by_name::load_by_name;
use crate::protocol::{encode_response, Request, EINVAL, ENOENT};
use crate::server::outcomes::Outcomes;

/// OP_LOAD_BY_NAME, its outcome kept for its sender (outcomes.rs).
pub fn load_noted(req: Request<'_>, sender_pid: u32, kept: &mut Outcomes) -> Vec<u8> {
    let payload = req.payload;
    let reply = load_by_name(req, sender_pid);
    let status = reply.get(4..8).map_or(EINVAL, |s| i32::from_le_bytes([s[0], s[1], s[2], s[3]]));
    if let Some(name) = name_of(payload) {
        kept.note(sender_pid, name, status);
    }
    reply
}

/// OP_LOAD_OUTCOME: the payload is a load's own (caps, name, args); the
/// status is how the sender's last load of that name ended, ENOENT when
/// none is kept. Served in order, so a load sent before it is over.
pub fn load_outcome(req: Request<'_>, sender_pid: u32, kept: &Outcomes) -> Vec<u8> {
    let status = match name_of(req.payload) {
        Some(name) => kept.of(sender_pid, name).unwrap_or(ENOENT),
        None => EINVAL,
    };
    encode_response(req.seq, status, &[])
}

/* A load's payload: requested_caps(8) | name_len(1) | name | args. */
fn name_of(p: &[u8]) -> Option<&[u8]> {
    let len = usize::from(*p.get(8)?);
    p.get(9..9 + len)
}
