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
//! Requests as the compositor sends them, through the wire server, and the
//! reply it would read.

use nonos_gfx_wire::{Header, Reply, HDR_LEN};

use super::rig::Rig;

pub const OWNER: u32 = 7;

/// The status and the body after it.
pub fn ask(r: &mut Rig, op: u16, output: u16, body: &[u8]) -> (i32, Vec<u8>) {
    let h = Header { op, flags: 0, output, request_id: 1, payload_len: body.len() as u32 };
    let mut f = vec![0u8; HDR_LEN];
    h.encode(&mut f).unwrap();
    f.extend_from_slice(body);
    let mut out = [0u8; 300];
    let n = r.s.handle(OWNER, &f, &mut out);
    let reply = Reply::decode(&out[..n], &h).expect("a reply to the request");
    (reply.status, reply.body.to_vec())
}

pub fn words(v: &[u32]) -> Vec<u8> {
    v.iter().flat_map(|x| x.to_le_bytes()).collect()
}
