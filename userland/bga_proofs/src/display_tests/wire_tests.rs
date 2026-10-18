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

//! Hostile frames: too short for a header, of another magic or version, or
//! a body other than the length the header names. Each is answered E_INVAL
//! under the request it names, since its sender waits for a reply, where
//! the same frame made well is taken: only the fault refuses it.

use super::frames::{answered, flush_body, frame};
use super::model::{served, take, COMPOSITOR};
use crate::serve::{E_INVAL, OP_FLUSH, OP_GET_PRIMARY_SURFACE};

/// A status-only reply as the compositor reads it: the request's header,
/// naming a 4-byte payload, then the status.
fn refused(op: u16, request_id: u32) -> (i32, Vec<u8>) {
    let mut reply = frame(op, request_id, &[]);
    reply[16..20].copy_from_slice(&4u32.to_le_bytes());
    reply.extend_from_slice(&E_INVAL.to_le_bytes());
    (E_INVAL, reply)
}

#[test]
fn a_frame_too_short_for_a_header_is_refused_under_zeros() {
    let mut s = served(64, 48);
    let whole = frame(OP_GET_PRIMARY_SURFACE, 7, &[]);
    for len in [0, 1, 4, 12, 19] {
        assert_eq!(answered(&mut s.d, COMPOSITOR, &whole[..len]), refused(0, 0), "{len}");
    }
}

#[test]
fn another_magic_or_version_is_refused_under_the_request_it_names() {
    let mut s = served(64, 48);
    let good = frame(OP_GET_PRIMARY_SURFACE, 7, &[]);
    for (at, byte) in [(0, 0x51), (3, 0), (4, 0), (4, 2), (5, 1)] {
        let mut f = good.clone();
        f[at] = byte;
        let want = refused(OP_GET_PRIMARY_SURFACE, 7);
        assert_eq!(answered(&mut s.d, COMPOSITOR, &f), want, "byte {at} as {byte:#x}");
    }
    assert_eq!(answered(&mut s.d, COMPOSITOR, &good).0, 0, "the frame itself was refused");
}

#[test]
fn a_body_other_than_the_length_its_header_names_is_refused() {
    let mut s = served(64, 48);
    take(&mut s);
    let good = frame(OP_FLUSH, 7, &flush_body(0, 0, 1, 1));
    let (mut short, mut long) = (good.clone(), good.clone());
    short.pop();
    long.push(0);
    let mut bad = vec![short, long];
    for named in [0u32, 1, 19, 21, 40, u32::MAX] {
        let mut f = good.clone();
        f[16..20].copy_from_slice(&named.to_le_bytes());
        bad.push(f);
    }
    for f in bad {
        assert_eq!(answered(&mut s.d, COMPOSITOR, &f), refused(OP_FLUSH, 7), "{f:02x?}");
    }
    assert_eq!(answered(&mut s.d, COMPOSITOR, &good).0, 0, "the frame itself was refused");
}
