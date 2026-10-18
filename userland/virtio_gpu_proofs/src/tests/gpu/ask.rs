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

use nonos_gfx_wire::{ops, Header, Reply, HDR_LEN};

use super::rig::Rig;
use crate::server::recover::settle;

pub const OWNER: u32 = 7;

/// The status and the body after it, the serve loop's next turn taken: a
/// stall the request hit is recovered (`settle`), as `run` does.
pub fn ask(r: &mut Rig, op: u16, output: u16, body: &[u8]) -> (i32, Vec<u8>) {
    let got = send(r, op, output, body);
    settle(&mut r.d);
    got
}

/// The request alone, the next turn not yet taken.
pub fn send(r: &mut Rig, op: u16, output: u16, body: &[u8]) -> (i32, Vec<u8>) {
    let h = Header { op, flags: 0, output, request_id: 1, payload_len: body.len() as u32 };
    let mut f = vec![0u8; HDR_LEN];
    h.encode(&mut f).unwrap();
    f.extend_from_slice(body);
    let mut out = [0u8; 300];
    let n = r.d.server.handle(OWNER, &f, &mut out);
    let reply = Reply::decode(&out[..n], &h).expect("a reply to the request");
    (reply.status, reply.body.to_vec())
}

pub fn words(v: &[u32]) -> Vec<u8> {
    v.iter().flat_map(|x| x.to_le_bytes()).collect()
}

/// SURFACE for `o`, as the compositor reads it.
pub fn surface(r: &mut Rig, o: u16) -> nonos_gfx_wire::Surface {
    nonos_gfx_wire::Surface::decode(&ask(r, ops::SURFACE, o, &[]).1).expect("a surface")
}
