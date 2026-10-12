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
//! One request answered. OPEN and START ask the kernel whether the peer
//! holds CameraCapture, every time; READ_FRAME and STOP are the streaming
//! peer's alone (gate.rs).

use crate::begin::begin;
use crate::device::Dev;
use crate::gate::CAMERA_CAPTURE;
use crate::listing::listing;
use crate::stream::{read_frame, stop};
use crate::wire::*;
use nonos_libc::mk_cap_check;

pub(crate) fn holds(peer: u32) -> bool {
    mk_cap_check(peer, CAMERA_CAPTURE)
}

pub fn answer(d: &mut Dev, peer: u32, req: &Request, body: &[u8], tx: &mut [u8]) -> usize {
    let r = |tx: &mut [u8], s: Result<(), i32>| reply(tx, req, s.err().unwrap_or(0), &[]);
    match (req.op, body) {
        (OP_HEALTHCHECK, []) => reply(tx, req, 0, &[]),
        (OP_OPEN, []) => match (d.session.open(holds(peer)), d.bound.as_ref()) {
            (Err(e), _) => reply(tx, req, e, &[]),
            (Ok(()), None) => reply(tx, req, E_NODEV, &[]),
            (Ok(()), Some(b)) => reply(tx, req, 0, &listing(&b.nic.cam, b.nic.ids, CHUNK)),
        },
        (OP_START, &[format, frame, _, _, i0, i1, i2, i3]) => {
            let interval = u32::from_le_bytes([i0, i1, i2, i3]);
            match begin(d, peer, (format, frame, interval)) {
                Ok(max) => reply(tx, req, 0, &max.to_le_bytes()),
                Err(e) => reply(tx, req, e, &[]),
            }
        }
        (OP_READ_FRAME, &[o0, o1, o2, o3]) => match d.session.own(peer) {
            Ok(()) => read_frame(d, req, u32::from_le_bytes([o0, o1, o2, o3]), tx),
            Err(e) => reply(tx, req, e, &[]),
        },
        (OP_STOP, []) => {
            let mine = d.session.own(peer);
            if mine.is_ok() {
                stop(d);
            }
            r(tx, mine)
        }
        _ => reply(tx, req, E_INVAL, &[]),
    }
}
