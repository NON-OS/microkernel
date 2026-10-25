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

//! The wire as the compositor speaks it: one request at a time, and a
//! driver primed the way the compositor primes one.

use crate::display::Svga;
use crate::pool::Pool;
use nonos_gfx_serve::Server;
use nonos_gfx_wire::{ops, Header, Reply, Surface, HDR_LEN};

/// The pid the compositor asks from.
pub const COMPOSITOR: u32 = 40;

/// One request from the compositor; the reply's status and body.
pub fn ask(s: &mut Server<Svga, Pool>, op: u16, payload: &[u8]) -> (i32, Vec<u8>) {
    let h = Header { op, flags: 0, output: 0, request_id: 1, payload_len: payload.len() as u32 };
    let mut f = vec![0u8; HDR_LEN];
    h.encode(&mut f).unwrap();
    f.extend_from_slice(payload);
    let mut out = [0u8; 300];
    let n = s.handle(COMPOSITOR, &f, &mut out);
    let r = Reply::decode(&out[..n], &h).unwrap();
    (r.status, r.body.to_vec())
}

pub fn words(v: &[u32]) -> Vec<u8> {
    v.iter().flat_map(|x| x.to_le_bytes()).collect()
}

/// CAPS then SURFACE, as the compositor primes a driver, and the primary's
/// memory painted with `px`.
pub fn primed(s: &mut Server<Svga, Pool>, px: u32) -> Surface {
    assert_eq!(ask(s, ops::CAPS, &[]).0, 0, "CAPS answered");
    let sf = Surface::decode(&ask(s, ops::SURFACE, &[]).1).unwrap();
    let base = s.slots[0].current.unwrap().region.base;
    for p in s.platform.memory.get_mut(&base).unwrap().chunks_exact_mut(4) {
        p.copy_from_slice(&px.to_le_bytes());
    }
    sf
}
