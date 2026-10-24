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

//! A request as the compositor sends it, and the reply read back.

use nonos_gfx_serve::{Display, Platform, Server};
use nonos_gfx_wire::{Header, Reply, HDR_LEN};

/// Send `op` with `payload` from `sender`; the reply's status and body.
pub fn ask<D: Display, P: Platform>(
    s: &mut Server<D, P>,
    sender: u32,
    op: u16,
    payload: &[u8],
) -> (i32, Vec<u8>) {
    let h = Header { op, flags: 0, output: 0, request_id: 5, payload_len: payload.len() as u32 };
    let mut f = vec![0u8; HDR_LEN];
    h.encode(&mut f).unwrap();
    f.extend_from_slice(payload);
    let mut out = [0u8; 300];
    let n = s.handle(sender, &f, &mut out);
    let r = Reply::decode(&out[..n], &h).expect("a reply to the request");
    (r.status, r.body.to_vec())
}
